## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsersResponse.ts)

## 示例

[inline-code-attrs-start title = 'getV2PageReactUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fc_tenant_7b4c9d1";
const rawUrlId: string | undefined = undefined; // 可能来自路由参数
const urlId: string = rawUrlId ?? "page-home-9a3f2b";
const id: string = "user_823b5c";

const response: GetV2PageReactUsersResponse = await getV2PageReactUsers(tenantId, urlId, id);
[inline-code-end]

---