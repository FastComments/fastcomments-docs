Wyświetla listę stron dla tenanta. Używane przez klienta desktopowego FChat do wypełnienia listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązanej niestandardowej konfiguracji dla każdej strony.
Strony wymagające SSO są filtrowane względem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
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
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]