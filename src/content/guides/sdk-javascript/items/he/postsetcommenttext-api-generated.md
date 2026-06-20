---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| setCommentTextParams | SetCommentTextParams | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentTextResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postSetCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_a1b2c3d4";
const setCommentTextParams: SetCommentTextParams = {
  text: "Updated timeline and added a reference to the original report for clarity.",
  editedByUserId: "user_9876",
  editReason: "clarified timeline and added source",
  editedAt: "2026-06-19T12:00:00Z"
};
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature";
const result: SetCommentTextResponse = await postSetCommentText(commentId, setCommentTextParams, sso);
[inline-code-end]

---