## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenant_id | String | Так |  |
| year_number | f64 | Ні |  |
| month_number | f64 | Ні |  |
| day_number | f64 | Ні |  |
| skip | f64 | Ні |  |

## Відповідь

Повертає: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_tenant_daily_usages Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_daily_usage(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetTenantDailyUsagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        year_number: Some(2023.0),
        month_number: Some(7.0),
        day_number: Some(15.0),
        skip: Some(0.0),
    };
    let _response: GetTenantDailyUsagesResponse = get_tenant_daily_usages(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---