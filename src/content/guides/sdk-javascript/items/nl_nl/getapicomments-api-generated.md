## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
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

## Respons

Retourneert: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getApiComments Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // pagina
    25,                    // aantal
    "feedback",           // tekstZoekopdracht
    "192.168.1.100",      // IPVanCommentaar
    "approved",           // filters
    "hasReplies",         // zoekFilters
    "dateDesc",           // sorteringen
    false,                // demo
    "tenant-abc123",      // tenantId
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]