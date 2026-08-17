## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |

## Одговор

Враћа: `GetV1PageLikes`

## Пример

[inline-code-attrs-start title = 'Пример get_v1_page_likes'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---