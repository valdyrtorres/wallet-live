use std::convert::Infallible;

use axum::extract::FromRequestParts;
use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::{
    app::AppState,
    models::{Asset, OwnedAsset, UserRecord},
};

pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub async fn list_assets(&self) -> sqlx::Result<Vec<Asset>> {
        sqlx::query_as!(
            Asset,
            "SELECT id, name, unit_value
             FROM assets;"
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn create_asset(&self, name: String, unit_value: f64) -> sqlx::Result<Asset> {
        sqlx::query_as!(
            Asset,
            "INSERT INTO assets (name, unit_value)
             VALUES ($1, $2)
             RETURNING id, name, unit_value;",
            name,
            unit_value
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn update_asset(
        &self,
        asset_id: i64,
        name: Option<String>,
        unit_value: Option<f64>,
    ) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as!(
            Asset,
            "UPDATE assets
             SET name=COALESCE($2, name),
                 unit_value=COALESCE($3, unit_value)
             WHERE id=$1
             RETURNING id, name, unit_value;",
            asset_id,
            name,
            unit_value
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn add_user(&self, username: &str, password_hash: &str) -> sqlx::Result<UserRecord> {
        sqlx::query_as!(
            UserRecord,
            "INSERT INTO users (username, password_hash)
             VALUES ($1, $2)
             RETURNING id, username, password_hash;",
            username,
            password_hash,
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn get_user_by_name(&self, username: &str) -> sqlx::Result<Option<UserRecord>> {
        sqlx::query_as!(
            UserRecord,
            "SELECT id, username, password_hash
             FROM users
             WHERE username = $1;",
            username
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn list_owned_assets(&self, user_id: i64) -> sqlx::Result<Vec<OwnedAsset>> {
        let rows = sqlx::query(
            r#"
            SELECT
                a.id,
                a.name,
                a.unit_value,
                COALESCE(SUM((a.unit_value - o.bought_for) * o.quantity_owned), 0.0) AS value_delta,
                COALESCE(SUM(o.quantity_owned), 0.0) AS quantity_owned,
                COALESCE(
                    JSON_AGG(
                        JSON_BUILD_OBJECT(
                            'bought_at', o.timestamp,
                            'bought_for', o.bought_for,
                            'quantity_bought', o.quantity_owned,
                            'value_delta', (a.unit_value - o.bought_for) * o.quantity_owned
                        )
                    ) FILTER (WHERE o.id IS NOT NULL),
                    '[]'::json
                ) AS purchase_history
            FROM assets AS a
            LEFT JOIN owned_assets AS o
            ON o.asset_id = a.id AND o.user_id = $1
            WHERE o.user_id = $1
            GROUP BY a.id, a.name, a.unit_value
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        let mut owned_assets = Vec::with_capacity(rows.len());

        for row in rows {
            let id: i64 = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let unit_value: f64 = row.try_get("unit_value")?;
            let value_delta: f64 = row.try_get("value_delta")?;
            let quantity_owned: f64 = row.try_get("quantity_owned")?;
            let purchase_history_json: Value = row.try_get("purchase_history")?;
            let purchase_history: Vec<crate::models::PurchaseHistory> =
                serde_json::from_value(purchase_history_json)
                    .map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

            owned_assets.push(OwnedAsset {
                id,
                name,
                unit_value,
                value_delta,
                quantity_owned,
                purchase_history,
            });
        }

        Ok(owned_assets)
    }

    pub async fn insert_owned_asset(
        &self,
        user_id: i64,
        asset_id: i64,
        quantity: f64,
        unit_value: f64,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO owned_assets
            (user_id, asset_id, quantity_owned, bought_for)
            VALUES ($1, $2, $3, $4)
            "#,
            user_id,
            asset_id,
            quantity,
            unit_value,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

}

impl FromRequestParts<AppState> for Repository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for Repository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
