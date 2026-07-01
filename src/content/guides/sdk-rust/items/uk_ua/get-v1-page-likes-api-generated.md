## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## Відповідь

Повертає: `GetV1PageLikes`

## Приклад

[inline-code-attrs-start title = 'get_v1_page_likes Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetV1PageLikesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
    };
    let _likes = get_v1_page_likes(configuration, params).await?;
    Ok(())
}
[inline-code-end]