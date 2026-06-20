## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| commentId | string | 是 |  |
| banEmail | boolean | 否 |  |
| banEmailDomain | boolean | 否 |  |
| banIP | boolean | 否 |  |
| deleteAllUsersComments | boolean | 否 |  |
| bannedUntil | string | 否 |  |
| isShadowBan | boolean | 否 |  |
| updateId | string | 否 |  |
| banReason | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## 範例

[inline-code-attrs-start title = 'postBanUserFromComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // 封鎖電子郵件
  false,       // 封鎖電子郵件網域
  true,        // 封鎖 IP
  true,        // 刪除此使用者的所有留言
  bannedUntil,
  false,       // 隱形封鎖
  updateId,
  banReason,
  sso
);
[inline-code-end]