## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | GetTenantDailyUsagesOptions | Не |  |

## Отговор

Връща: [`Option[GetTenantDailyUsagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getTenantDailyUsages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.getTenantDailyUsages(
  tenantId = "my-tenant-123",
  options = default(GetTenantDailyUsagesOptions),
)
if respOpt.isSome:
  let usage = respOpt.get()
  echo usage
  echo httpResp.statusCode
[inline-code-end]