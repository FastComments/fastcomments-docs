## 参数

| 名称 | 类型 | 必需 | 说明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| replaceTenantUserBody | ReplaceTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'replaceTenantUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9d8f4b2c';
const id: string = 'user_f47ac10b';
const replaceTenantUserBody: ReplaceTenantUserBody = {
  externalId: 'ext-5234',
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  roles: ['moderator'],
  metadata: { department: 'product', region: 'us-east-1' }
};
const updateComments: string = 'propagate-display-name-to-comments';

const result: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---