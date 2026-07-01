## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|-------------|------|
| tenant_id | String | Так |  |
| id | String | Так |  |

## Відповідь

Повертає: [`DeletePageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_page_api_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_page'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---