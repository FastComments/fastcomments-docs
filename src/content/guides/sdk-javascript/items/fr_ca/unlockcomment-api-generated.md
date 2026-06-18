## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne : [`LockComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockComment200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de unLockComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-8f3b2c4a";
const commentId: string = "cmt_92a7f3e6";
const broadcastId: string = "brd_1b4c9d20";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const result: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---