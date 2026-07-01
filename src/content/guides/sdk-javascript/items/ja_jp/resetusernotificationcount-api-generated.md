## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|------|------|
| tenantId | string | はい |  |
| sso | string | いいえ |  |

## Response

戻り値: [`ResetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationCountResponse.ts)

## Example

[inline-code-attrs-start title = 'resetUserNotificationCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoResetCount() {
  const tenantId: string = "acme-corp-tenant";
  const sso: string = "sso-user-9876";

  // オプションの sso パラメータで呼び出す
  const resultWithSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId, sso);

  // オプションの sso パラメータなしで呼び出す
  const resultWithoutSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId);

  console.log(resultWithSso, resultWithoutSso);
}
[inline-code-end]