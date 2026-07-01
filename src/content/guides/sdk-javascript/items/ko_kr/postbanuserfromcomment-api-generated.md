## 매개변수

| 이름 | 유형 | 필수 | 설명 |
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
| tenantId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`PostBanUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBanUserFromCommentResponse.ts)

## 예시

[inline-code-attrs-start title = 'postBanUserFromComment 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runBan() {
  const commentId: string = "cmt_5f8a2b3c";
  const banEmail: boolean = true;
  const banIP: boolean = false;
  const deleteAllUsersComments: boolean = true;
  const bannedUntil: string = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString();
  const isShadowBan: boolean = false;
  const banReason: string = "Repeated spam posting";
  const tenantId: string = "tenant_12345";

  const response: PostBanUserFromCommentResponse = await postBanUserFromComment(
    commentId,
    banEmail,
    undefined,
    banIP,
    deleteAllUsersComments,
    bannedUntil,
    isShadowBan,
    undefined,
    banReason,
    tenantId
  );
  console.log(response);
}
runBan();
[inline-code-end]