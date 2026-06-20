## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| textSearch | string | Nej |  |
| byIPFromComment | string | Nej |  |
| filters | string | Nej |  |
| searchFilters | string | Nej |  |
| afterId | string | Nej |  |
| demo | boolean | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentIdsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getApiIds'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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