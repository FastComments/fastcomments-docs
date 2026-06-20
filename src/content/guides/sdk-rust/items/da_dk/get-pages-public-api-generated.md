Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde dens rumsliste.
Kræver, at `enableFChat` er true i den resulterende brugerdefinerede konfiguration for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| cursor | String | Nej |  |
| limit | i32 | Nej |  |
| q | String | Nej |  |
| sort_by | models::PagesSortBy | Nej |  |
| has_comments | bool | Nej |  |

## Svar

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_pages_public Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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