## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id_ws | String | Da |  |
| user_ids | String | Da |  |

## Odgovor

Vrne: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_presence_statuses_response.rs)

## Primer

[inline-code-attrs-start title = 'get_user_presence_statuses Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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