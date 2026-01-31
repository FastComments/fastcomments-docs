## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| limit | f64 | No |  |
| skip | f64 | No |  |

## Response

Returns: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_list_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badge_progress_list Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_user_badge_progress() -> Result<(), Error> {
    let params: GetUserBadgeProgressListParams = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-7812".to_string()),
        limit: Some(25.0),
        skip: Some(0.0),
    };
    let badge_progress: GetUserBadgeProgressList200Response =
        get_user_badge_progress_list(&configuration, params).await?;
    println!("{:#?}", badge_progress);
    Ok(())
}
[inline-code-end]
