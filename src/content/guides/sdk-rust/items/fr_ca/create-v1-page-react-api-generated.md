## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| title | String | Non |  |

## Réponse

Retourne : `CreateV1PageReact`

## Exemple

[inline-code-attrs-start title = 'Exemple create_v1_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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