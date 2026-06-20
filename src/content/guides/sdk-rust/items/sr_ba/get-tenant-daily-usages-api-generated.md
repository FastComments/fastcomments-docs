---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| year_number | f64 | Не |  |
| month_number | f64 | Не |  |
| day_number | f64 | Не |  |
| skip | f64 | Не |  |

## Одговор

Враћа: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_response.rs)

## Пример

[inline-code-attrs-start title = 'get_tenant_daily_usages Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantDailyUsagesParams = GetTenantDailyUsagesParams {
        tenant_id: String::from("acme-corp-tenant"),
        year_number: Some(2026.0),
        month_number: Some(6.0),
        day_number: Some(19.0),
        skip: Some(0.0),
    };
    let daily_usages: GetTenantDailyUsagesResponse =
        get_tenant_daily_usages(&configuration, params).await?;
    let _ = daily_usages;
    Ok(())
}
[inline-code-end]

---