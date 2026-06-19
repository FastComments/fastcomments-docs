## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Oui |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |

## Réponse

Renvoie : [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de unBlockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8b4a2f9c';
const id: string = 'cmt_5f3b2a9e';
const unBlockFromCommentParams: UnBlockFromCommentParams = { reason: 'Appeal accepted', effectiveAt: '2026-06-19T12:00:00Z' };
const userId: string = 'user_42f7';
const result: UnblockSuccess = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId);
[inline-code-end]

---