## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| banEmail | boolean | いいえ |  |
| banEmailDomain | boolean | いいえ |  |
| banIP | boolean | いいえ |  |
| deleteAllUsersComments | boolean | いいえ |  |
| bannedUntil | string | いいえ |  |
| isShadowBan | boolean | いいえ |  |
| updateId | string | いいえ |  |
| banReason | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## 例

[inline-code-attrs-start title = 'postBanUserFromComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (省略)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (省略)
    "Harassment",             // banReason
    undefined                 // sso (省略)
  );

  console.log(result);
}
[inline-code-end]

---