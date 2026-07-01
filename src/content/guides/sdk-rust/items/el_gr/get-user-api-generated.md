## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Απάντηση

Επιστρέφει: [`GetUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'get_user Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-123".to_string(),
        include_details: Some(true),
    };
    let _response = get_user(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---