## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | number | 否 |  |
| count | number | 否 |  |
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filters | string | 否 |  |
| searchFilters | string | 否 |  |
| sorts | string | 否 |  |
| demo | boolean | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getApiComments 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // 页码
    25,                    // 条数
    "feedback",           // 文本搜索
    "192.168.1.100",      // 来自评论的IP
    "approved",           // 过滤器
    "hasReplies",         // 搜索过滤器
    "dateDesc",           // 排序
    false,                // 演示
    "tenant-abc123",      // 租户ID
    "sso-token-xyz"       // 单点登录
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]