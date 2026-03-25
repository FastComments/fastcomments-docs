## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Da |  |

## Odgovor

Vraća: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "u_987654",
  planId: "pro_monthly",
  startDate: new Date().toISOString(),
  trialDays: 14, // neobavezni parametar (primjer)
  metadata: { source: "marketing-email" } // neobavezni parametar (primjer)
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
const subscription: APIUserSubscription = result.subscription;
[inline-code-end]

---