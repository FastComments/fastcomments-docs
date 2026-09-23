## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentIds | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckBlockedCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple checkedCommentsForBlocked'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "tenant_12345";
    const commentIds: string = "cmt_9876,cmt_5432";
    const ssoToken: string = "sso_user_abc123";

    const responseWithSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
    const responseWithoutSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds);
})();
[inline-code-end]

---