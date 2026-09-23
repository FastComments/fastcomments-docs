## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| contextUserId | string | Nein |  |
| isLive | boolean | Nein |  |

## Antwort

Rückgabe: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResult.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeComment(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const commentId: string = "comment_9876";
    const contextUserId: string = "user_42";
    const result: DeleteCommentResult = await deleteComment(tenantId, commentId, contextUserId, true);
    console.log(result);
}
removeComment();
[inline-code-end]

---