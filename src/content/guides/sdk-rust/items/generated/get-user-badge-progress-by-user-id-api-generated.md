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
async fn run() -> Result<(), Error> {
    let params: GetUserBadgeProgressByUserIdParams = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "journalist_987".to_string(),
    };
    let progress: GetUserBadgeProgressById200Response =
        get_user_badge_progress_by_user_id(&configuration, params).await?;
    println!("{:#?}", progress);
    Ok(())
}
[inline-code-end]
