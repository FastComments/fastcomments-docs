## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| skip | i32 | Ne |  |

## Odgovor

Vrne: [`GetSsoUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_users_response.rs)

## Primer

[inline-code-attrs-start title = 'get_sso_users Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetSsoUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10),
    };
    let _response = get_sso_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---