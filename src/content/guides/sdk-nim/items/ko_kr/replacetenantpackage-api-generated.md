## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | 아니요 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'replaceTenantPackage 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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