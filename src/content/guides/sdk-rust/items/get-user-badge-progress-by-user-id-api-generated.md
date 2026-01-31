## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_by_id_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_user_badge_progress(configuration: &configuration::Configuration) -> Result<GetUserBadgeProgressById200Response, Error> {
    let params: GetUserBadgeProgressByUserIdParams = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "journalist-212".to_string(),
    };
    let include_inactive: Option<bool> = Some(false);
    let badge_progress: GetUserBadgeProgressById200Response = get_user_badge_progress_by_user_id(configuration, params).await?;
    Ok(badge_progress)
}
[inline-code-end]
