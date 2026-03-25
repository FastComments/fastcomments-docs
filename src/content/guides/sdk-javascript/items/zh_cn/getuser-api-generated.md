## 参数

| 名称 | Type | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## Response

返回: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## 示例

[inline-code-attrs-start title = 'getUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b3f42';
const id: string = 'user_9c4d2a';
const userResponse: GetUser200Response = await getUser(tenantId, id);
console.log(userResponse);
[inline-code-end]

---