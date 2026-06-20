## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| textSearch | string | Ні |  |
| byIPFromComment | string | Ні |  |
| filters | string | Ні |  |
| searchFilters | string | Ні |  |
| afterId | string | Ні |  |
| demo | boolean | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentIdsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getApiIds'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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