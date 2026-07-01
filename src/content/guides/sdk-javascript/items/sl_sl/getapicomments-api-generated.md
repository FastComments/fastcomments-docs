## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| page | number | No |  |
| count | number | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Response

Vrne: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Example

[inline-code-attrs-start title = 'Primer getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // stran
    25,                    // število
    "feedback",           // iskanje besedila
    "192.168.1.100",      // poIPIzKomentarja
    "approved",           // filtri
    "hasReplies",         // filtriIskanja
    "dateDesc",           // razvrščanje
    false,                // demo
    "tenant-abc123",      // ID najemnika
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]