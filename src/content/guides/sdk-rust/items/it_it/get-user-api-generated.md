## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |

## Risposta

Restituisce: [`GetUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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