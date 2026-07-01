## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |

## Odpowiedź

Zwraca: [`DeletePageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_page_api_response.rs)

## Przykład

[inline-code-attrs-start title = 'delete_page Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = DeletePageParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "news/article".into(),
    };
    let _resp = delete_page(configuration, params).await?;
    Ok(())
}
[inline-code-end]