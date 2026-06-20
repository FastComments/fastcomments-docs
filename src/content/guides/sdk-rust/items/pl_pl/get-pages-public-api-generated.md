Wyświetla listę stron dla najemcy. Używane przez desktopowego klienta FChat do wypełnienia jego listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w ostatecznej niestandardowej konfiguracji dla każdej strony.
Strony wymagające SSO są filtrowane w oparciu o dostęp grupowy użytkownika wysyłającego żądanie.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| cursor | String | Nie |  |
| limit | i32 | Nie |  |
| q | String | Nie |  |
| sort_by | models::PagesSortBy | Nie |  |
| has_comments | bool | Nie |  |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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