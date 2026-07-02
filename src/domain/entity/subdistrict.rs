use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;
use super::AuditMetadata;

/// Strongly-typed ID for Subdistrict
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubdistrictId(pub Uuid);

impl SubdistrictId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for SubdistrictId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SubdistrictId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for SubdistrictId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<SubdistrictId> for Uuid {
    fn from(id: SubdistrictId) -> Self { id.0 }
}

impl AsRef<Uuid> for SubdistrictId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for SubdistrictId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subdistrict {
    pub id: Uuid,
    pub name: String,
    pub country_id: Uuid,
    pub province_id: Uuid,
    pub city_id: Uuid,
    pub district_id: Uuid,
    pub postal_code: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Subdistrict {
    /// Create a builder for Subdistrict
    pub fn builder() -> SubdistrictBuilder {
        SubdistrictBuilder::default()
    }

    /// Create a new Subdistrict with required fields
    pub fn new(name: String, country_id: Uuid, province_id: Uuid, city_id: Uuid, district_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            country_id,
            province_id,
            city_id,
            district_id,
            postal_code: None,
            latitude: None,
            longitude: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> SubdistrictId {
        SubdistrictId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the postal_code field (chainable)
    pub fn with_postal_code(mut self, value: String) -> Self {
        self.postal_code = Some(value);
        self
    }

    /// Set the latitude field (chainable)
    pub fn with_latitude(mut self, value: Decimal) -> Self {
        self.latitude = Some(value);
        self
    }

    /// Set the longitude field (chainable)
    pub fn with_longitude(mut self, value: Decimal) -> Self {
        self.longitude = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "country_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.country_id = v; }
                }
                "province_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.province_id = v; }
                }
                "city_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.city_id = v; }
                }
                "district_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.district_id = v; }
                }
                "postal_code" => {
                    if let Ok(v) = serde_json::from_value(value) { self.postal_code = v; }
                }
                "latitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.latitude = v; }
                }
                "longitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.longitude = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Subdistrict {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Subdistrict"
    }
}

impl backbone_core::PersistentEntity for Subdistrict {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for Subdistrict {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("country_id".to_string(), "uuid".to_string());
        m.insert("province_id".to_string(), "uuid".to_string());
        m.insert("city_id".to_string(), "uuid".to_string());
        m.insert("district_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name"]
    }
    fn relations() -> &'static [(&'static str, &'static str, &'static str)] {
        &[("district", "districts", "districtId")]
    }
}

/// Builder for Subdistrict entity
///
/// Provides a fluent API for constructing Subdistrict instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct SubdistrictBuilder {
    name: Option<String>,
    country_id: Option<Uuid>,
    province_id: Option<Uuid>,
    city_id: Option<Uuid>,
    district_id: Option<Uuid>,
    postal_code: Option<String>,
    latitude: Option<Decimal>,
    longitude: Option<Decimal>,
}

impl SubdistrictBuilder {
    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the country_id field (required)
    pub fn country_id(mut self, value: Uuid) -> Self {
        self.country_id = Some(value);
        self
    }

    /// Set the province_id field (required)
    pub fn province_id(mut self, value: Uuid) -> Self {
        self.province_id = Some(value);
        self
    }

    /// Set the city_id field (required)
    pub fn city_id(mut self, value: Uuid) -> Self {
        self.city_id = Some(value);
        self
    }

    /// Set the district_id field (required)
    pub fn district_id(mut self, value: Uuid) -> Self {
        self.district_id = Some(value);
        self
    }

    /// Set the postal_code field (optional)
    pub fn postal_code(mut self, value: String) -> Self {
        self.postal_code = Some(value);
        self
    }

    /// Set the latitude field (optional)
    pub fn latitude(mut self, value: Decimal) -> Self {
        self.latitude = Some(value);
        self
    }

    /// Set the longitude field (optional)
    pub fn longitude(mut self, value: Decimal) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Build the Subdistrict entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Subdistrict, String> {
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let country_id = self.country_id.ok_or_else(|| "country_id is required".to_string())?;
        let province_id = self.province_id.ok_or_else(|| "province_id is required".to_string())?;
        let city_id = self.city_id.ok_or_else(|| "city_id is required".to_string())?;
        let district_id = self.district_id.ok_or_else(|| "district_id is required".to_string())?;

        Ok(Subdistrict {
            id: Uuid::new_v4(),
            name,
            country_id,
            province_id,
            city_id,
            district_id,
            postal_code: self.postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            metadata: AuditMetadata::default(),
        })
    }
}
