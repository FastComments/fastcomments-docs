## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| textSearch | string | 아니요 |  |
| byIPFromComment | string | 아니요 |  |
| filters | string | 아니요 |  |
| searchFilters | string | 아니요 |  |
| afterId | string | 아니요 |  |
| demo | boolean | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentIdsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getApiIds 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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