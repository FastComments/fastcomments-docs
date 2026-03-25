## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Да |  |

## Response

Возвращает: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "u_987654",
  planId: "pro_monthly",
  startDate: new Date().toISOString(),
  trialDays: 14, // необязательный параметр (продемонстрировано)
  metadata: { source: "marketing-email" } // необязательный параметр (продемонстрировано)
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
const subscription: APIUserSubscription = result.subscription;
[inline-code-end]

---