## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| page | number | Nein |  |
| count | number | Nein |  |
| textSearch | string | Nein |  |
| byIPFromComment | string | Nein |  |
| filters | string | Nein |  |
| searchFilters | string | Nein |  |
| sorts | string | Nein |  |
| demo | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getApiComments Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const page: number = 2;
  const count: number = 20;
  const textSearch: string = "typescript";
  const byIPFromComment: string = "192.168.1.100";
  const filters: string = "spam,offensive";
  const searchFilters: string = "user:john";
  const sorts: string = "date_desc";
  const demo: boolean = true;
  const sso: string = "sso_token_abc";

  const response: ModerationAPIGetCommentsResponse = await getApiComments(
    tenantId,
    page,
    count,
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    sorts,
    demo,
    sso
  );
}

fetchComments();
[inline-code-end]

---