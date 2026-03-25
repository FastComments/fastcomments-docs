## Parametri

| Ime | Vrsta | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Da |  |

## Odgovor

Vrne: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "u_987654",
  planId: "pro_monthly",
  startDate: new Date().toISOString(),
  trialDays: 14, // neobvezen parameter (prikazan)
  metadata: { source: "marketing-email" } // neobvezen parameter (prikazan)
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
const subscription: APIUserSubscription = result.subscription;
[inline-code-end]