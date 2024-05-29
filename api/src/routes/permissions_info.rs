use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

pub const _ROUTE: &str = "/permissions_info";
pub const _PERMISSIONS: &str = "NONE";
pub const _TYPE: &str = "GET";

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
	specific: Option<String>,
}

pub async fn callback(Query(query): Query<QueryParams>) -> Json<Vec<crate::structs::permission_flags_info::PermissionFlagsInfo>> {
	if let Some(specific) = query.specific {
		let specific_vec: Vec<&str> = specific.split(',').collect();
		return Json(
			crate::PERMISSION_FLAGS_INFO
				.clone()
				.into_iter()
				.filter(|permission_info| specific_vec.contains(&permission_info.get_flag()))
				.collect::<Vec<crate::structs::permission_flags_info::PermissionFlagsInfo>>(),
		);
	}
	Json(crate::PERMISSION_FLAGS_INFO.to_vec())
}
