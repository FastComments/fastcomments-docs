## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| limit | f64 | No |  |
| skip | f64 | No |  |

## Antwort

Gibt zurück: [`ApiGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_list_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_user_badge_progress_list'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetUserBadgeProgressListParams = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-9876".to_string()),
        limit: Some(25.0),
        skip: Some(0.0),
    };
    let badge_progress: ApiGetUserBadgeProgressListResponse =
        get_user_badge_progress_list(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---