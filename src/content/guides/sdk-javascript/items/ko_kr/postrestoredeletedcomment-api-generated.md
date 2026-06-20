## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'postRestoreDeletedComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = 'cmt_7f3b2a1e-54d3';
  const resultWithoutSso: APIEmptyResponse = await postRestoreDeletedComment(commentId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSIsImlhdCI6MTYyMzQ1Njc4OX0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
  const resultWithSso: APIEmptyResponse = await postRestoreDeletedComment(commentId, ssoToken);
  console.log(resultWithoutSso, resultWithSso);
})();
[inline-code-end]

---