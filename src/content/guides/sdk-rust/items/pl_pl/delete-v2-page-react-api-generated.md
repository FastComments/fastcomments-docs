## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| id | String | Tak |  |

## Odpowiedź

Zwraca: `CreateV1PageReact`

## Przykład

[inline-code-attrs-start title = 'delete_v2_page_react Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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