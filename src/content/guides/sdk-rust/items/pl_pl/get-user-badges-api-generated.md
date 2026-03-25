## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| user_id | String | Nie |  |
| badge_id | String | Nie |  |
| displayed_on_comments | bool | Nie |  |
| limit | f64 | Nie |  |
| skip | f64 | Nie |  |

## Odpowiedź

Zwraca: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badges_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_user_badges'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserBadgesParams = GetUserBadgesParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("user-9876")),
        badge_id: Some(String::from("top-reviewer")),
        displayed_on_comments: Some(true),
        limit: Some(50.0),
        skip: Some(0.0),
    };
    let response: GetUserBadges200Response = get_user_badges(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---