## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
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

## 예시

[inline-code-attrs-start title = 'postBanUserFromComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (생략됨)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (생략됨)
    "Harassment",             // banReason
    undefined                 // sso (생략됨)
  );

  console.log(result);
}
[inline-code-end]