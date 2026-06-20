## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| sso | string | No |  |

## 응답

반환: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentTextResponse.ts)

## 예제

[inline-code-attrs-start title = 'getModerationCommentText 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f3a2b7d6e1c4a5b";
const ssoToken: string | undefined = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NDMyMSJ9.DUMMY_SIGNATURE";
const commentResponse: GetCommentTextResponse = await getModerationCommentText(commentId);
const commentResponseWithSso: GetCommentTextResponse = await getModerationCommentText(commentId, ssoToken);
[inline-code-end]