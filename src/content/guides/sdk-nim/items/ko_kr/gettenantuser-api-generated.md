## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[GetTenantUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_user200response.nim)

## 예제

[inline-code-attrs-start title = 'getTenantUser 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUser(tenantId = "my-tenant-123", id = "user-456")
if response.isSome:
  let tenantUser = response.get()
  discard tenantUser
else:
  discard httpResponse
[inline-code-end]

---