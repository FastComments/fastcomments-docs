## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Ja |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |

## Svar

Returnerer: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## Eksempel

[inline-code-attrs-start title = 'unBlockUserFromComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8b7c6d";
  const commentId: string = "comment-4a3b2c1d";
  const params: UnBlockFromCommentParams = {
    reason: "User appealed the block",
    adminNote: "Reviewed and unblocked"
  };
  const userId: string = "user-5e6f7g8h";
  const anonUserId: string = "anon-1a2b3c4d";
  const result: UnblockSuccess = await unBlockUserFromComment(
    tenantId,
    commentId,
    params,
    userId,
    anonUserId
  );
  console.log(result);
})();
[inline-code-end]