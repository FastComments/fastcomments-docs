## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| user_id | String | Ja |  |

## Antwort

Gibt zurück: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let user_id_opt: Option<&str> = Some("user-7823");
    let params: GetUserBadgeProgressByUserIdParams = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: user_id_opt.unwrap().to_string(),
    };
    let response: ApiGetUserBadgeProgressResponse =
        get_user_badge_progress_by_user_id(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---