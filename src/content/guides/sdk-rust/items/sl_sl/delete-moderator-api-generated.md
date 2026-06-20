## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| send_email | String | Ne |  |

## Odgovor

Vrača: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer delete_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteModeratorParams = DeleteModeratorParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("moderator-93b1f"),
        send_email: Some(String::from("moderator@acme-corp.com")),
    };
    let _response: ApiEmptyResponse = delete_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---