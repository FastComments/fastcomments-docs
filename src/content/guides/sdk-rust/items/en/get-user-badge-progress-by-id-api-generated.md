## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_by_id_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badge_progress_by_id Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_badge_progress() -> Result<(), Error> {
    let params: GetUserBadgeProgressByIdParams = GetUserBadgeProgressByIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-7f3a2e9".to_string(),
        include_history: Some(true),
    };
    let progress: GetUserBadgeProgressById200Response =
        get_user_badge_progress_by_id(configuration, params).await?;
    println!("{:#?}", progress);
    Ok(())
}
[inline-code-end]
