## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getCommentChildren 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "5f8d0a2e-3b9f-4c2b-9a37-1f4e6b2c7d8f";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1Njc4OSJ9.signature";
const childrenResponse: ModerationAPIChildCommentsResponse = await getCommentChildren(commentId);
const childrenResponseWithSso: ModerationAPIChildCommentsResponse = await getCommentChildren(commentId, sso);
[inline-code-end]

---