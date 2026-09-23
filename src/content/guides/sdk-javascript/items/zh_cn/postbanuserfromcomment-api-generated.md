## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| banEmail | boolean | No |  |
| banEmailDomain | boolean | No |  |
| banIP | boolean | No |  |
| deleteAllUsersComments | boolean | No |  |
| bannedUntil | string | No |  |
| isShadowBan | boolean | No |  |
| updateId | string | No |  |
| banReason | string | No |  |
| sso | string | No |  |

## 响应

返回：[`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## 示例

[inline-code-attrs-start title = 'postBanUserFromComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (已省略)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (已省略)
    "Harassment",             // banReason
    undefined                 // sso (已省略)
  );

  console.log(result);
}
[inline-code-end]