## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| urlId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'putCloseThread 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const urlId: string = 'thread-2f7c9b6a';
const closeResultWithoutSSO: APIEmptyResponse = await putCloseThread(urlId);

const urlIdWithSSO: string = 'thread-8a9b3e1c';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiNjc4OSJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const closeResultWithSSO: APIEmptyResponse = await putCloseThread(urlIdWithSSO, ssoToken);
[inline-code-end]

---