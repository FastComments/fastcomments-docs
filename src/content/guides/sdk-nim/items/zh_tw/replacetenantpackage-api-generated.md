## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | 否 |  |

## 回應

回傳: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'replaceTenantPackage 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.replaceTenantPackage(
  tenantId = "my-tenant-123",
  id = "pkg-987",
  replaceTenantPackageBody = ReplaceTenantPackageBody(
    name = "Premium Plan",
    priceCents = 999,
    seats = 50,
    enabled = true,
    features = @["moderation", "analytics", "priority-support"]
  )
)

if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---