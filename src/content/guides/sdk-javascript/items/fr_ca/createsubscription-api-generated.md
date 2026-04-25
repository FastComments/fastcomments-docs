## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Oui |  |

## Réponse

Renvoie : [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "user_98765",
  planId: "pro_monthly",
  paymentMethod: { type: "card", cardId: "card_abc123" },
  autoRenew: true,
  trialDays: 14, // paramètre optionnel démontré
  metadata: { campaign: "spring_launch" } // paramètre optionnel démontré
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
[inline-code-end]