---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantUserBody | UpdateTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'updateTenantUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f4a2b7c";
const id: string = "user_74d9c1a3";
const updateTenantUserBody: UpdateTenantUserBody = {
  email: "jane.doe@acme-corp.com",
  displayName: "Jane Doe",
  roles: ["moderator"],
  active: true
};
const updateComments: string = "Promoted to moderator for customer support";
const response: APIEmptyResponse = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]

---