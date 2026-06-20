## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| user_id | String | Non |  |
| anon_user_id | String | Non |  |

## Réponse

Renvoie : [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'un_flag_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn unflag_example() -> Result<FlagCommentResponse, Error> {
    let params: UnFlagCommentParams = UnFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-98765".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let response: FlagCommentResponse = un_flag_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---