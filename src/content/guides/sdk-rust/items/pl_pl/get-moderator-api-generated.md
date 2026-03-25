## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |

## Odpowiedź

Zwraca: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-4521".to_string(),
        include_permissions: Some(true),
    };
    let moderator: GetModerator200Response = get_moderator(&configuration, params).await?;
    println!("{:#?}", moderator);
    Ok(())
}
[inline-code-end]

---