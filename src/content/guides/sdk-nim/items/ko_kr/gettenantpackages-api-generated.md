---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | float64 | 아니오 |  |

## 응답

반환: [`Option[GetTenantPackages_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_packages200response.nim)

## 예제

[inline-code-attrs-start title = 'getTenantPackages 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackages(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let packages = response.get()
  echo "Received packages for tenant:", " my-tenant-123"
  echo packages
else:
  echo "No packages found, status:", httpResponse.status
[inline-code-end]

---