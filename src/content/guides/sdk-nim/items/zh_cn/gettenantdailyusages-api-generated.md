## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| options | GetTenantDailyUsagesOptions | 否 |  |

## 响应

返回：[`Option[GetTenantDailyUsagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages_response.nim)

## 示例

[inline-code-attrs-start title = 'getTenantDailyUsages 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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