## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | 是 |  |

## 回應

回傳：[`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## 範例

[inline-code-attrs-start title = 'createSubscription 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "user_98765",
  planId: "pro_monthly",
  paymentMethod: { type: "card", cardId: "card_abc123" },
  autoRenew: true,
  trialDays: 14, // 示範可選參數
  metadata: { campaign: "spring_launch" } // 示範可選參數
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
[inline-code-end]