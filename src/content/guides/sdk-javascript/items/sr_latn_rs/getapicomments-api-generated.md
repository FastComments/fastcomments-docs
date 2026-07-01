## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| page | number | Ne |  |
| count | number | Ne |  |
| textSearch | string | Ne |  |
| byIPFromComment | string | Ne |  |
| filters | string | Ne |  |
| searchFilters | string | Ne |  |
| sorts | string | Ne |  |
| demo | boolean | Ne |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // page
    25,                    // count
    "feedback",           // textSearch
    "192.168.1.100",      // byIPFromComment
    "approved",           // filters
    "hasReplies",         // searchFilters
    "dateDesc",           // sorts
    false,                // demo
    "tenant-abc123",      // tenantId
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]