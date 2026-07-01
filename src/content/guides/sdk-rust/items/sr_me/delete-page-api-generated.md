## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Одговор

Враћа: [`DeletePageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_page_api_response.rs)

## Пример

[inline-code-attrs-start title = 'delete_page Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = DeletePageParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "news/article".into(),
    };
    let _resp = delete_page(configuration, params).await?;
    Ok(())
}
[inline-code-end]