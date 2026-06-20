---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetLogsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getLogs 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = 'cmt_0f9b1a2c3d4e5f6a';
  const logsWithoutSSO: ModerationAPIGetLogsResponse = await getLogs(commentId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1Njc4OSJ9.dQw4w9WgXcQ';
  const logsWithSSO: ModerationAPIGetLogsResponse = await getLogs(commentId, ssoToken);
  console.log(logsWithoutSSO, logsWithSSO);
})();
[inline-code-end]

---