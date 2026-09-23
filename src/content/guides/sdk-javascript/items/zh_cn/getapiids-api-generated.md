## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filters | string | 否 |  |
| searchFilters | string | 否 |  |
| afterId | string | 否 |  |
| demo | boolean | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentIdsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getApiIds 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-corp";
  const textSearch: string = "offensive language";
  const byIPFromComment: string = "203.0.113.45";
  const filters: string = "status:unmoderated";
  const searchFilters: string = "author:alice";
  const afterId: string = "cmt_00123";
  const demo: boolean = true;
  const sso: string = "sso_9f8e7d6c";

  const fullResult: ModerationAPIGetCommentIdsResponse = await getApiIds(
    tenantId,
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    afterId,
    demo,
    sso
  );

  const minimalResult: ModerationAPIGetCommentIdsResponse = await getApiIds(tenantId);
}
[inline-code-end]