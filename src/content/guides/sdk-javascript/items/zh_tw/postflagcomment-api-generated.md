## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'postFlagComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const response: APIEmptyResponse = await postFlagComment('cmt_8f3b2a1f4e6');
const responseWithSso: APIEmptyResponse = await postFlagComment('cmt_9b4a7c2d5f1', 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NzEyMzQ1NiIsImlhdCI6MTYyNzcxMzYwMH0.sig-token-part');
[inline-code-end]

---