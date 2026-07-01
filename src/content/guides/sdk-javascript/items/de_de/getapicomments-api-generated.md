## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| page | number | Nein |  |
| count | number | Nein |  |
| textSearch | string | Nein |  |
| byIPFromComment | string | Nein |  |
| filters | string | Nein |  |
| searchFilters | string | Nein |  |
| sorts | string | Nein |  |
| demo | boolean | Nein |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getApiComments Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // Seite
    25,                    // Anzahl
    "feedback",           // Textsuche
    "192.168.1.100",      // vonIPausKommentar
    "approved",           // Filter
    "hasReplies",         // Suchfilter
    "dateDesc",           // Sortierungen
    false,                // Demo
    "tenant-abc123",      // Mandanten-ID
    "sso-token-xyz"       // SSO
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]