## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Ja |  |

## Antwoord

Retourneert: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createSubscription Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "u_987654",
  planId: "pro_monthly",
  startDate: new Date().toISOString(),
  trialDays: 14, // optionele parameter ter demonstratie
  metadata: { source: "marketing-email" } // optionele parameter ter demonstratie
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
const subscription: APIUserSubscription = result.subscription;
[inline-code-end]