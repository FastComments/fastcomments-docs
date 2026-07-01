## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[GetTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_response.nim)

## 예시

[inline-code-attrs-start title = 'getTenant 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (tenantResponse, httpResponse) = client.getTenant(tenantId = "my-tenant-123", id = "config-001")
if tenantResponse.isSome:
  let tenant = tenantResponse.get()
  echo tenant
[inline-code-end]