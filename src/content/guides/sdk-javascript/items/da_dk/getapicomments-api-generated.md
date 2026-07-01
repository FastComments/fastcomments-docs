## Parametre

| Navn | Type | Obligatory | Beskrivelse |
|------|------|------------|-------------|
| page | number | Nej |  |
| count | number | Nej |  |
| textSearch | string | Nej |  |
| byIPFromComment | string | Nej |  |
| filters | string | Nej |  |
| searchFilters | string | Nej |  |
| sorts | string | Nej |  |
| demo | boolean | Nej |  |
| tenantId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returns: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getApiComments Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // side
    25,                    // antal
    "feedback",           // tekstSøgning
    "192.168.1.100",      // efterIPFraKommentar
    "approved",           // filtre
    "hasReplies",         // søgeFiltre
    "dateDesc",           // sorteringer
    false,                // demo
    "tenant-abc123",      // lejerId
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]

---