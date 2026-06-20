## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| commentId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUserInternalProfile 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const profileByCommentId: GetUserInternalProfileResponse = await getUserInternalProfile('comment_5f1e8a3b9c2d4');
const profileBySSOToken: GetUserInternalProfileResponse = await getUserInternalProfile(undefined, 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummypayload.signature');
[inline-code-end]

---