## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| user_id | String | Non |  |
| anon_user_id | String | Non |  |

## Réponse

Renvoie : [`FlagComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de un_flag_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_unflag_comment() -> Result<FlagComment200Response, Error> {
    let params = UnFlagCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article/comment-12345"),
        user_id: Some(String::from("reader-987")),
        anon_user_id: None,
    };
    let response: FlagComment200Response = un_flag_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---