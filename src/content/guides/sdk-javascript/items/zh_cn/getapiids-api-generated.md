## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
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
const textSearch: string = "promo link to discounted sneakers";
const byIPFromComment: string | undefined = "203.0.113.45";
const filters: string | undefined = "status:pending,is_spam:true";
const searchFilters: string | undefined = "created_at>=2026-01-01";
const afterId: string | undefined = "cmt_9a7b3d";
const demo: boolean | undefined = false;
const sso: string | undefined = undefined;
const result: ModerationAPIGetCommentIdsResponse = await getApiIds(textSearch, byIPFromComment, filters, searchFilters, afterId, demo, sso);
[inline-code-end]

---