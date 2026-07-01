## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentIds | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`CheckedCommentsForBlockedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckedCommentsForBlockedResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple checkedCommentsForBlocked'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const commentIds: string = "cmt_001,cmt_002";
  const ssoToken: string = "ssoTokenXYZ";

  const blockedCheck: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds);
  const blockedCheckWithSso: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
})();
[inline-code-end]