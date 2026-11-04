Наводи странице за закупца (tenant). Користи FChat десктоп клијент да попуни листу својих соба.
Захтева да `enableFChat` буде true у резолваној прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају у односу на приступ групама тражеоца.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| cursor | String | Не |  |
| limit | i32 | Не |  |
| q | String | Не |  |
| sort_by | models::PagesSortBy | Не |  |
| has_comments | bool | Не |  |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Пример

[inline-code-attrs-start title = 'get_pages_public Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]