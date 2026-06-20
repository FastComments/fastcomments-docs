## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| post_ids | Vec<String> | Да |  |
| sso | String | Не |  |

## Одговор

Враћа: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/feed_posts_stats_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_feed_posts_stats'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_main(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetFeedPostsStatsParams = GetFeedPostsStatsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: vec![
            "news/2026-product-launch".to_string(),
            "blog/engineering-architecture".to_string()
        ],
        sso: Some("sso.jwt.token.eyJhbGci...".to_string()),
    };
    let stats: FeedPostsStatsResponse = get_feed_posts_stats(configuration, params).await?;
    let _api_status: ApiStatus = /* користите статистику по потреби */ stats.into();
    Ok(())
}
[inline-code-end]

---