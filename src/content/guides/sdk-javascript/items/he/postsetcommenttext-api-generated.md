## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| setCommentTextParams | SetCommentTextParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## תגובה

מחזיר: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentTextResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת postSetCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const setCommentTextParams: SetCommentTextParams = {
  text: "Revised comment after user edit.",
  format: "plain"
};
const broadcastId: string = "broadcast_2023_09";

const response: SetCommentTextResponse = await postSetCommentText(
  tenantId,
  commentId,
  setCommentTextParams,
  broadcastId
);

console.log(response);
[inline-code-end]