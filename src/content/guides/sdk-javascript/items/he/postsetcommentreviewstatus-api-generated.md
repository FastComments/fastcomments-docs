## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | כן |  |
| reviewed | boolean | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postSetCommentReviewStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentIdSimple: string = "cmt_93a2b1d4";
const resultSimple: APIEmptyResponse = await postSetCommentReviewStatus(commentIdSimple);

const commentIdWithOptions: string = "cmt_7a8f2b6c";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0MjMifQ.signature";
const resultWithOptions: APIEmptyResponse = await postSetCommentReviewStatus(commentIdWithOptions, true, ssoToken);
[inline-code-end]

---