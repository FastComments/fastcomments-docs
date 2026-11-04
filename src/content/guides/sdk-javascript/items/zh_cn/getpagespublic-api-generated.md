列出租户的页面。由 FChat 桌面客户端用于填充其房间列表。
要求每个页面的已解析自定义配置中 `enableFChat` 为 true。
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| cursor | string | 否 |  |
| limit | number | 否 |  |
| q | string | 否 |  |
| sortBy | PagesSortBy | 否 |  |
| hasComments | boolean | 否 |  |

## 响应

返回:[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]