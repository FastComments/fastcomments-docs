## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vrača: [`GetUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_user() -> Result<(), Error> {
    let params = GetUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-7b9a2c".to_string(),
        include_roles: Some(true),
    };
    let user: GetUserResponse = get_user(&configuration, params).await?;
    println!("{:#?}", user);
    Ok(())
}
[inline-code-end]

---