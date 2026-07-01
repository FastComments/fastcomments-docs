## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| reviewed | boolean | לא |  |
| broadcastId | string | לא |  |
| tenantId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`PostSetCommentReviewStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentReviewStatusResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'postSetCommentReviewStatus דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updateCommentReviewStatus(): Promise<void> {
  const commentId: string = "cmt_9f8a7b6c5d4e3f2a1b0c";
  const reviewed: boolean = true;
  const broadcastId: string = "broadcast_2024Q1";
  const tenantId: string = "tenant_1001";
  const sso: string = "alice@example.com";

  const response: PostSetCommentReviewStatusResponse = await postSetCommentReviewStatus(
    commentId,
    reviewed,
    broadcastId,
    tenantId,
    sso
  );

  console.log(response);
}
[inline-code-end]