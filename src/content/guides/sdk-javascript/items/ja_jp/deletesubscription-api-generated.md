## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|------|------|
| tenantId | string | はい |  |
| id | string | はい |  |
| userId | string | いいえ |  |

## 応答

返却: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSubscriptionAPIResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteSubscription 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9876";
  const subscriptionId: string = "sub_54321";
  const userId: string = "user_abc123";

  // オプションの userId を使用
  const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId, userId);

  // オプションの userId なし
  const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId);
})();
[inline-code-end]