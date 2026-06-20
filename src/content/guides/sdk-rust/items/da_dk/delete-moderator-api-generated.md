---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| send_email | String | Nej |  |

## Svar

Returnerer: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Eksempel

[inline-code-attrs-start title = 'delete_moderator Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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