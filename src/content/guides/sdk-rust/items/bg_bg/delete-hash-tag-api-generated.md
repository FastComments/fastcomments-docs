## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | String | Да |  |
| tenant_id | String | Не |  |
| delete_hash_tag_request | models::DeleteHashTagRequest | Не |  |

## Отговор

Връща: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за delete_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_tag(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params = DeleteHashTagParams {
        tag: "news/world-climate".to_owned(),
        tenant_id: Some("acme-corp-tenant".to_owned()),
        delete_hash_tag_request: None,
    };
    let response: FlagCommentPublic200Response = delete_hash_tag(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---