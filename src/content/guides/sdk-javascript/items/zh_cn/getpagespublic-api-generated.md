---
列出租户的页面。由 FChat 桌面客户端使用，以填充其房间列表。每个页面的解析自定义配置中 `enableFChat` 必须为 true。需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

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

返回：[`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]

---