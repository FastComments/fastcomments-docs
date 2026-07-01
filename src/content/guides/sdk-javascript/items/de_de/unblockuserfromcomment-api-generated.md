## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------------|----------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Ja |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Antwort

Rückgabe: [`UnBlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockUserFromCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'unBlockUserFromComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUnblock() {
  const tenantId: string = "acme-corp-tenant";
  const commentId: string = "cmt_9f8b7a6d";

  const params: UnBlockFromCommentParams = {
    reason: "User resolved the issue",
    notifyUser: true
  };

  const userId: string = "usr_12345";

  const result: UnBlockUserFromCommentResponse = await unBlockUserFromComment(
    tenantId,
    commentId,
    params,
    userId
    // anonUserId omitted
  );

  console.log(result);
}
demoUnblock();
[inline-code-end]