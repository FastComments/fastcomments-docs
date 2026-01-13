## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | String | Так |  |
| tenant_id | String | Ні |  |
| delete_hash_tag_request | models::DeleteHashTagRequest | Ні |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteHashTagParams = DeleteHashTagParams {
        tag: "news/politics".to_string(),
        tenant_id: Some("acme-corp-tenant".to_string()),
        delete_hash_tag_request: Some(models::DeleteHashTagRequest::default()),
    };
    let response: FlagCommentPublic200Response = delete_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---