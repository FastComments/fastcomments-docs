## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersFromCommentResponse.ts)

## 예제

[inline-code-attrs-start title = 'getBanUsersFromComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = "cmt_7f4d2a9b6c";
  const ssoToken: string = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature";
  const bannedWithoutSso: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  const bannedWithSso: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(commentId, ssoToken);
  console.log(bannedWithoutSso, bannedWithSso);
})();
[inline-code-end]

---