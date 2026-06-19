列出租户的页面。FChat 桌面客户端用于填充其房间列表。
要求已解析的每个页面的自定义配置中的 `enableFChat` 为 true。
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## 响应

返回: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---