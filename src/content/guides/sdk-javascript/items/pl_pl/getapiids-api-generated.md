## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| textSearch | string | Nie |  |
| byIPFromComment | string | Nie |  |
| filters | string | Nie |  |
| searchFilters | string | Nie |  |
| afterId | string | Nie |  |
| demo | boolean | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentIdsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getApiIds'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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