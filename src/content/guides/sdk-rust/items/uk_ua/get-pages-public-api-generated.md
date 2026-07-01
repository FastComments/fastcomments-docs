Список сторінок для орендаря. Використовується настільним клієнтом FChat для заповнення списку його кімнат.  
Вимагає, щоб `enableFChat` був встановлений у true у розв’язаній кастомній конфігурації для кожної сторінки.  
Сторінки, які вимагають SSO, фільтруються згідно з груповим доступом запитуючого користувача.

## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| cursor | String | No |  |
| limit | i32 | No |  |
| q | String | No |  |
| sort_by | models::PagesSortBy | No |  |
| has_comments | bool | No |  |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_pages_public Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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