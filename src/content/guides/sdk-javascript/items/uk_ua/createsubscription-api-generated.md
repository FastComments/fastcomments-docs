## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Так |  |

## Відповідь

Повертає: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "u_987654",
  planId: "pro_monthly",
  startDate: new Date().toISOString(),
  trialDays: 14, // необов'язковий параметр (показано)
  metadata: { source: "marketing-email" } // необов'язковий параметр (показано)
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
const subscription: APIUserSubscription = result.subscription;
[inline-code-end]