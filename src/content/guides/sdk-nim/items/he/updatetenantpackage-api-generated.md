## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| updateTenantPackageBody | UpdateTenantPackageBody | לא |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let packageBody = UpdateTenantPackageBody(
  name: "Pro Plan",
  priceCents: 1999,
  active: true,
  features: @["priority-support", "advanced-moderation"]
)

let (response, httpResponse) = client.updateTenantPackage(
  tenantId = "my-tenant-123",
  id = "pkg-789",
  updateTenantPackageBody = packageBody
)

if response.isSome:
  let apiEmpty = response.get()
  echo "Tenant package updated successfully, HTTP status: " & $httpResponse.status
[inline-code-end]

---