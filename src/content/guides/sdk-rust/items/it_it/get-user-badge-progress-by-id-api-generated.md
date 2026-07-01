## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Risposta

Restituisce: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio get_user_badge_progress_by_id'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetUserBadgeProgressByIdParams {
        tenant_id: "acme-corp".to_string(),
        id: "user-12345".to_string(),
    };
    let _response = get_user_badge_progress_by_id(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---