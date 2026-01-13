## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |

## 응답

반환: [`Option[GetTenant_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant200response.nim)

## 예제

[inline-code-attrs-start title = 'getTenant 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenant(tenantId = "my-tenant-123", id = "")
if response.isSome:
  let tenant = response.get()
  echo "Tenant retrieved"
  discard tenant
else:
  echo "No tenant found"
  echo "HTTP status:", httpResponse.status
[inline-code-end]

---