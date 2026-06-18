## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## 示例

[inline-code-attrs-start title = 'getUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments_corp';
const id: string = 'user_9f8b7c6d-5e4a-3b2c-1f0e-123456789abc';
const response: GetUser200Response = await getUser(tenantId, id);
const userEmail: string | undefined = response.user?.email;
const displayName: string | undefined = response.user?.displayName
[inline-code-end]

---