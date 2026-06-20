## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| value | String | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_user_search_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_search_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_search() -> Result<(), Error> {
    let params: GetSearchUsersParams = GetSearchUsersParams {
        value: Some("jane.doe@acme.com".to_string()),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let user_search: ModerationUserSearchResponse = get_search_users(&configuration, params).await?;
    let _ = user_search;
    Ok(())
}
[inline-code-end]

---