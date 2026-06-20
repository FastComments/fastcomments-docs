---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| badges_user_id | String | Non |  |
| comment_id | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_manual_badges_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_manual_badges_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_manual_badges() -> Result<GetUserManualBadgesResponse, Error> {
    let params: GetManualBadgesForUserParams = GetManualBadgesForUserParams {
        badges_user_id: Some(String::from("acme-user-42")),
        comment_id: Some(String::from("news/article-5678")),
        sso: Some(String::from("sso-token-abc123")),
    };
    let response: GetUserManualBadgesResponse = get_manual_badges_for_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---