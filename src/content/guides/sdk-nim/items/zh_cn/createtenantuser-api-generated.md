## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantUserBody | CreateTenantUserBody | 否 |  |

## 响应

返回: [`Option[CreateTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user_response.nim)

## 示例

[inline-code-attrs-start title = 'createTenantUser 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---