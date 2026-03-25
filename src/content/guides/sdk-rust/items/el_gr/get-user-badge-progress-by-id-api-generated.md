## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_progress_by_id_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_user_badge_progress_by_id'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_badge_progress() -> Result<GetUserBadgeProgressById200Response, Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params = GetUserBadgeProgressByIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-007-community-builder".to_string(),
    };
    let response: GetUserBadgeProgressById200Response = get_user_badge_progress_by_id(cfg, params).await?;
    Ok(response)
}
[inline-code-end]