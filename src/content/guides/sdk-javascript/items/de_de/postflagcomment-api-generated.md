## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Nein |  |
| sso | string | Nein |  |

## Response

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postFlagComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";

  // Nur erforderliche Parameter
  const result1: APIEmptyResponse = await postFlagComment(tenantId, commentId);

  // Including optional parameters
  const broadcastId: string = "brd_54321";
  const sso: string = "user@example.com";
  const result2: APIEmptyResponse = await postFlagComment(tenantId, commentId, broadcastId, sso);

  console.log(result1, result2);
}
runExample();
[inline-code-end]