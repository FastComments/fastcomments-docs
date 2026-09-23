## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Risposta

Restituisce: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio unFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt-20230915-001";
  const userId: string = "user-42";

  const unflaggedByUser: FlagCommentResponse = await unFlagComment(tenantId, commentId, userId);
  const unflaggedByAnon: FlagCommentResponse = await unFlagComment(tenantId, commentId, undefined, "anon-xyz");

  console.log(unflaggedByUser, unflaggedByAnon);
}
[inline-code-end]