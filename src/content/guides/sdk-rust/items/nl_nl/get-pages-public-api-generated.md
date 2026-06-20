Lijst pagina's voor een tenant. Wordt gebruikt door de FChat desktopclient om de kamerlijst te vullen.
Vereist dat `enableFChat` true is in de opgeloste aangepaste configuratie voor elke pagina.
Pagina's die SSO vereisen worden gefilterd op basis van de groepstoegang van de gebruiker die het verzoek doet.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| cursor | String | Nee |  |
| limit | i32 | Nee |  |
| q | String | Nee |  |
| sort_by | models::PagesSortBy | Nee |  |
| has_comments | bool | Nee |  |

## Response

Geeft terug: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_pages_public Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]

---