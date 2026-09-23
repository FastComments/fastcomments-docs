## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |

## Svar

Returnerer: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'unFlagComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---