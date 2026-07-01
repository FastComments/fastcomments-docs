## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| url_id_ws | String | Yes |  |
| user_ids | String | Yes |  |

## Odgovor

Vraća: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_presence_statuses_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer get_user_presence_statuses'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetUserPresenceStatusesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id_ws: "news/article".to_string(),
        user_ids: "user123,user456".to_string(),
    };
    let _response = get_user_presence_statuses(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---