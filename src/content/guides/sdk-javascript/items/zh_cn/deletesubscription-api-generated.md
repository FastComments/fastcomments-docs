## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| userId | string | 否 |  |

## 响应

返回: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSubscriptionAPIResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteSubscription 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9876";
  const subscriptionId: string = "sub_54321";
  const userId: string = "user_abc123";

  // 使用可选的 userId
  const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId, userId);

  // 未使用可选的 userId
  const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId);
})();
[inline-code-end]

---