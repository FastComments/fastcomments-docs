## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| id | String | Yes |  |

## Odgovor

Vraća: `CreateV1PageReact`

## Primer

[inline-code-attrs-start title = 'delete_v2_page_react Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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