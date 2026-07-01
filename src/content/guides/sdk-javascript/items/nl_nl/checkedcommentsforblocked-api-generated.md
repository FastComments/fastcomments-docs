## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| commentIds | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`CheckedCommentsForBlockedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckedCommentsForBlockedResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'checkedCommentsForBlocked Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const commentIds: string = "cmt_001,cmt_002";
  const ssoToken: string = "ssoTokenXYZ";

  const blockedCheck: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds);
  const blockedCheckWithSso: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
})();
[inline-code-end]