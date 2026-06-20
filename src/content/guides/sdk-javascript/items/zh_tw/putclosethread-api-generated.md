---
## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| urlId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'putCloseThread 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const urlId: string = 'thread-2f7c9b6a';
const closeResultWithoutSSO: APIEmptyResponse = await putCloseThread(urlId);

const urlIdWithSSO: string = 'thread-8a9b3e1c';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiNjc4OSJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const closeResultWithSSO: APIEmptyResponse = await putCloseThread(urlIdWithSSO, ssoToken);
[inline-code-end]

---