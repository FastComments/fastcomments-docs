## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vraća: [`GetUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_user Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_user() -> Result<(), Error> {
    let maybe_id: Option<String> = Some("user-6412".to_owned());
    let params: GetUserParams = GetUserParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        id: maybe_id.unwrap(),
    };
    let user_response: GetUser200Response = get_user(&configuration, params).await?;
    println!("{:#?}", user_response);
    Ok(())
}
[inline-code-end]

---