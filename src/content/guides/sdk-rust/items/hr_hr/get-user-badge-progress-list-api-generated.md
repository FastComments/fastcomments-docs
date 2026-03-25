## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| user_id | String | Ne |  |
| limit | f64 | Ne |  |
| skip | f64 | Ne |  |

## Odgovor

Vraća: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_list_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_user_badge_progress_list Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserBadgeProgressListParams = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user_98765".to_string()),
        limit: Some(25.0),
        skip: Some(0.0),
    };
    let response: GetUserBadgeProgressList200Response =
        get_user_badge_progress_list(&configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---