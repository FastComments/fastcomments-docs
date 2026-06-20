---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| urlId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'putReopenThread 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const reopenResultWithSso: APIEmptyResponse = await putReopenThread("th_3c9b2a7f", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjEyM30.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c");
const reopenResultNoSso: APIEmptyResponse = await putReopenThread("th_7a4e5c1d");
[inline-code-end]

---