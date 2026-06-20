## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | לא |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת replaceTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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