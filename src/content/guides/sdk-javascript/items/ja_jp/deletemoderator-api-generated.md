## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sendEmail | string | No |  |

## レスポンス

返り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteModerator の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const moderatorId: string = "mod_9876";
  const notifyEmail: string = "admin@company.com";

  const resultWithEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId, notifyEmail);
  const resultWithoutEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId);
})();
[inline-code-end]