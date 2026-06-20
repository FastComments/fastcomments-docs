## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tag | String | Da |  |
| tenant_id | String | Ne |  |
| delete_hash_tag_request_body | models::DeleteHashTagRequestBody | Ne |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primer

[inline-code-attrs-start title = 'delete_hash_tag Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteHashTagParams = DeleteHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    delete_hash_tag_request_body: Some(DeleteHashTagRequestBody {}),
};
let response: ApiEmptyResponse = delete_hash_tag(&configuration, params).await?;
[inline-code-end]

---