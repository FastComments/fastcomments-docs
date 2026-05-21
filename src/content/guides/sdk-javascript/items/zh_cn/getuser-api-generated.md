## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## 示例

[inline-code-attrs-start title = 'getUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const idSuffix: string | undefined = undefined;
const tenantId: string = "acme-enterprises";
const id: string = idSuffix ?? "user_98765";
const response: GetUser200Response = await getUser({ tenantId, id });
[inline-code-end]

---