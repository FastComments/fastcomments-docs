## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createTenantUserBody | CreateTenantUserBody | いいえ |  |

## レスポンス

戻り値: [`Option[CreateTenantUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user200response.nim)

## 例

[inline-code-attrs-start title = 'createTenantUser の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateTenantUserBody(
  email = "jane.doe@acmepub.com",
  displayName = "Jane Doe",
  disabled = false
)
let (response, httpResponse) = client.createTenantUser(tenantId = "my-tenant-123", createTenantUserBody = createBody)
if response.isSome:
  let createdUser = response.get()
  echo createdUser
[inline-code-end]

---