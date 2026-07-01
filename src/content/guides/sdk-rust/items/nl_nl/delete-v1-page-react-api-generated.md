## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |

## Respons

Retourneert: `CreateV1PageReact`

## Voorbeeld

[inline-code-attrs-start title = 'delete_v1_page_react voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(cfg: &configuration::Configuration) -> Result<(), Error> {
    let tenant_id: String = Some("acme-corp-tenant".to_string()).unwrap();
    let url_id: String = "news/article".to_string();
    let params: DeleteV1PageReactParams = DeleteV1PageReactParams {
        tenant_id,
        url_id,
        ..Default::default()
    };
    let _result = delete_v1_page_react(cfg, params).await?;
    Ok(())
}
[inline-code-end]