## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| tag | String | Sim |  |
| delete_hash_tag_request_body | models::DeleteHashTagRequestBody | Não |  |

## Resposta

Retorna: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo delete_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteHashTagParams {
        tenant_id: "acme-corp-tenant".to_string(),
        tag: "news/article".to_string(),
        delete_hash_tag_request_body: Some(models::DeleteHashTagRequestBody {}),
    };
    delete_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---