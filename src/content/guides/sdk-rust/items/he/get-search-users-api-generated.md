## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| value | String | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_user_search_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_search_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("john.doe".to_string()),
        sso: Some("sso-provider".to_string()),
    };
    let _response: ModerationUserSearchResponse = get_search_users(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---