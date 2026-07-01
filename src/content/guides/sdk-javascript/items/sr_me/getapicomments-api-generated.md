## Parametri

| Ime | Tip | Obavezno | Opis |
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

## Odgovor

Vraća: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Primer

[inline-code-attrs-start title = 'getApiComments Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // strana
    25,                    // broj
    "feedback",           // pretragaTeksta
    "192.168.1.100",      // poIPuIzKomentara
    "approved",           // filteri
    "hasReplies",         // filteriPretrage
    "dateDesc",           // sortiranje
    false,                // demo
    "tenant-abc123",      // tenantId
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]