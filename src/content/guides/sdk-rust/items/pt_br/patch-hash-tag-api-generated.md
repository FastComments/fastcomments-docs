## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| tag | String | Sim |  |
| update_hash_tag_body | models::UpdateHashTagBody | Não |  |

## Resposta

Retorna: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo patch_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PatchHashTagParams {
        tenant_id: "acme-corp-tenant".into(),
        tag: "news/article".into(),
        update_hash_tag_body: Some(models::UpdateHashTagBody::default()),
    };
    let _response = patch_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]