Osoby komentujące w przeszłości na stronie, które NIE są obecnie online. Sortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję "Members".
Paginacja kursorowa po commenterName: serwer przechodzi przez częściowy indeks {tenantId, urlId, commenterName}
od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| after_name | String | Nie |  |
| after_user_id | String | Nie |  |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]