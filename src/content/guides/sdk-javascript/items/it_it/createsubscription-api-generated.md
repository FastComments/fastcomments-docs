## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Sì |  |

## Risposta

Restituisce: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "user_98765",
  planId: "pro_monthly",
  paymentMethod: { type: "card", cardId: "card_abc123" },
  autoRenew: true,
  trialDays: 14, // parametro opzionale dimostrato
  metadata: { campaign: "spring_launch" } // parametro opzionale dimostrato
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
[inline-code-end]

---