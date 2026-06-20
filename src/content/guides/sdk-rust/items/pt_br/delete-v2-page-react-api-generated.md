## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| id | String | Sim |  |

## Resposta

Retorna: `CreateV1PageReact`

## Exemplo

[inline-code-attrs-start title = 'Exemplo de delete_v2_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let params: DeleteV2PageReactParams = DeleteV2PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2026/06/feature-ml".to_string(),
        id: "react_987654321".to_string(),
    };
    let request_id: Option<String> = Some("req-20260619-01".to_string());
    let deleted: CreateV1PageReact = delete_v2_page_react(&configuration, params).await?;
    let _ = request_id;
    Ok(())
}
[inline-code-end]

---