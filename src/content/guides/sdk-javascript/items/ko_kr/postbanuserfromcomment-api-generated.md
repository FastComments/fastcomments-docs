## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| banEmail | boolean | 아니오 |  |
| banEmailDomain | boolean | 아니오 |  |
| banIP | boolean | 아니오 |  |
| deleteAllUsersComments | boolean | 아니오 |  |
| bannedUntil | string | 아니오 |  |
| isShadowBan | boolean | 아니오 |  |
| updateId | string | 아니오 |  |
| banReason | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## 예제

[inline-code-attrs-start title = 'postBanUserFromComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // 이메일 차단
  false,       // 이메일 도메인 차단
  true,        // IP 차단
  true,        // 사용자의 모든 댓글 삭제
  bannedUntil,
  false,       // 그림자 차단
  updateId,
  banReason,
  sso
);
[inline-code-end]

---