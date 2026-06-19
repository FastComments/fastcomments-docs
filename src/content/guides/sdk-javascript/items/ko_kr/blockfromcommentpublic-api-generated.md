## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## 예제

[inline-code-attrs-start title = 'blockFromCommentPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_52b9f3a1";
const commentId: string = "cmt_4f9d2a7b";
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: "spam",
  moderatorId: "mod_783",
  durationMinutes: 1440,
  notifyUser: true
};
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example";
const result: BlockSuccess = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---