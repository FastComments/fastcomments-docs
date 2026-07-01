## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | No |  |

## 応答

返り値: [`Option[CreateTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user_response.nim)

## 例

[inline-code-attrs-start title = 'createTenantUser の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userBody = CreateTenantUserBody(
  email: "john.doe@example.com",
  name: "John Doe",
  password: "s3cr3tP@ss",
  role: "admin"
)
let (optResp, httpResp) = client.createTenantUser(
  tenantId = tenantId,
  createTenantUserBody = userBody
)
if optResp.isSome:
  let resp = optResp.get()
  echo resp.userId
[inline-code-end]