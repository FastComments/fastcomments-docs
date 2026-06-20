## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
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

## 响应

返回: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## 示例

[inline-code-attrs-start title = 'postBanUserFromComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // 封禁电子邮件
  false,       // 封禁电子邮件域
  true,        // 封禁 IP
  true,        // 删除该用户的所有评论
  bannedUntil,
  false,       // 是否为影子封禁
  updateId,
  banReason,
  sso
);
[inline-code-end]

---