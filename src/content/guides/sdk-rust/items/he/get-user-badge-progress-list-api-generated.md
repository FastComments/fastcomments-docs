## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| limit | f64 | No |  |
| skip | f64 | No |  |

## תגובה

מחזיר: [`ApiGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_list_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל‑get_user_badge_progress_list'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badge_progress(conf: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-98765".to_string()),
        limit: Some(20.0),
        skip: Some(5.0),
    };
    let _resp = get_user_badge_progress_list(conf, params).await?;
    Ok(())
}
[inline-code-end]