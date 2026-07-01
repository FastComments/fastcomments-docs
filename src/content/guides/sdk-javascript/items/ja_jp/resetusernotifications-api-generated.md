## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| afterId | string | いいえ |  |
| afterCreatedAt | number | いいえ |  |
| unreadOnly | boolean | いいえ |  |
| dmOnly | boolean | いいえ |  |
| noDm | boolean | いいえ |  |
| sso | string | いいえ |  |

## 応答

返却: [`ResetUserNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse1.ts)

## 例

[inline-code-attrs-start title = 'resetUserNotifications の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-001";
  const afterId: string = "notif-123";
  const afterCreatedAt: number = 1697049600; // 例の UNIX タイムスタンプ
  const unreadOnly: boolean = true;
  const dmOnly: boolean = false;
  const noDm: boolean = false;
  const sso: string = "sso-token-xyz";

  const result: ResetUserNotificationsResponse1 = await resetUserNotifications(
    tenantId,
    afterId,
    afterCreatedAt,
    unreadOnly,
    dmOnly,
    noDm,
    sso
  );

  console.log(result);
})();
[inline-code-end]