Списък на страниците за даден tenant. Използва се от FChat десктоп клиента за попълване на списъка с му стаи.
Изисква `enableFChat` да е true в резолвнатата персонализирана конфигурация за всяка страница.
Страниците, изискващи SSO, се филтрират спрямо груповия достъп на заявяващия потребител.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| cursor | String | Не |  |
| limit | i32 | Не |  |
| q | String | Не |  |
| sort_by | models::PagesSortBy | Не |  |
| has_comments | bool | Не |  |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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