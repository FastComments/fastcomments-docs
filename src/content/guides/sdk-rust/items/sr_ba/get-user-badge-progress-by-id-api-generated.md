## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vraća: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer get_user_badge_progress_by_id'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserBadgeProgressByIdParams = GetUserBadgeProgressByIdParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        id: "badge-gold-2026".to_owned(),
        user_id: Some("user-987".to_owned()),
    };
    let badge_progress: ApiGetUserBadgeProgressResponse =
        get_user_badge_progress_by_id(&configuration, params).await?;
    println!("{:#?}", badge_progress);
    Ok(())
}
[inline-code-end]

---