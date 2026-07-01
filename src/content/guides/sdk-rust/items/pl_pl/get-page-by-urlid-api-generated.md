---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## Odpowiedź

Zwraca: [`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## Przykład

[inline-code-attrs-start title = 'get_page_by_urlid Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetPageByUrlidParams {
        tenant_id: "acme-corp-tenant".into(),
        url_id: "news/article".into(),
    };
    let _response = get_page_by_urlid(&config, params).await?;
    Ok(())
}
[inline-code-end]

---