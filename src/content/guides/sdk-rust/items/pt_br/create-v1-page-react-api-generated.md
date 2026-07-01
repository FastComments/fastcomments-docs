## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| title | String | Não |  |

## Response

Retorna: `CreateV1PageReact`

## Example

[inline-code-attrs-start title = 'create_v1_page_react Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateV1PageReactParams = CreateV1PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        title: Some("Rust Community Update".to_string()),
    };
    let _response = create_v1_page_react(&config, params).await?;
    Ok(())
}
[inline-code-end]