## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| contextUserId | string | नहीं |  |
| isLive | boolean | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResult.ts)

## उदाहरण

[inline-code-attrs-start title = 'deleteComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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