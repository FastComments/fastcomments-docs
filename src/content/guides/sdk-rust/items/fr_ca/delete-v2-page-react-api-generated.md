## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| id | String | Oui |  |

## Réponse

Retourne: `CreateV1PageReact`

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_v2_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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