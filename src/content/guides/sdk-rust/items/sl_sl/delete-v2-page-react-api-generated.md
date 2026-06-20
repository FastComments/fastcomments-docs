---
## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vrača: `CreateV1PageReact`

## Primer

[inline-code-attrs-start title = 'Primer delete_v2_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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