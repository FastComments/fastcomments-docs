## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
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

[inline-code-attrs-start title = 'getApiIds Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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