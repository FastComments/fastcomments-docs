## パラメーター

| Name | Type | 必須 | 説明 |
|------|------|----------|-------------|
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

戻り値: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## 例

[inline-code-attrs-start title = 'postBanUserFromComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // banEmail（メールアドレスを禁止する）
  false,       // banEmailDomain（メールドメインを禁止する）
  true,        // banIP（IPを禁止する）
  true,        // deleteAllUsersComments（ユーザーの全コメントを削除する）
  bannedUntil,
  false,       // isShadowBan（シャドウバンにする）
  updateId,
  banReason,
  sso
);
[inline-code-end]

---