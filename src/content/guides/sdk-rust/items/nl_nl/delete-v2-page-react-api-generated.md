## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Retourneert: `CreateV1PageReact`

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld delete_v2_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteV2PageReactParams {
        tenant_id: "acme-corp-tenant".into(),
        url_id: "news/article".into(),
        id: "react-987".into(),
    };
    let _response: CreateV1PageReact = delete_v2_page_react(&config, params).await?;
    Ok(())
}
[inline-code-end]

---