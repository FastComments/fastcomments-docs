## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| context_user_id | String | Non |  |
| is_live | bool | Non |  |

## Réponse

Renvoie : [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple delete_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn main() -> Result<(), Error> {
    let params = DeleteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        context_user_id: Some("user-6789".to_string()),
        is_live: Some(true),
    };
    let _result = delete_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]