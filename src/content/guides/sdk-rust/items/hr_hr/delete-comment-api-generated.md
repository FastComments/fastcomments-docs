## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| context_user_id | String | Ne |  |
| is_live | bool | Ne |  |

## Odgovor

Vraća: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## Primjer

[inline-code-attrs-start title = 'delete_comment Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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