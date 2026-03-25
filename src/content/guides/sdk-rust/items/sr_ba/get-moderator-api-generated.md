## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vraća: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_moderator Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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