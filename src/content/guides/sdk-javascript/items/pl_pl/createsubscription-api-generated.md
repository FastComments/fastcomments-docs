## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Tak |  |

## Odpowiedź

Zwraca: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "u_987654",
  planId: "pro_monthly",
  startDate: new Date().toISOString(),
  trialDays: 14, // opcjonalny parametr (przykład)
  metadata: { source: "marketing-email" } // opcjonalny parametr (przykład)
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
const subscription: APIUserSubscription = result.subscription;
[inline-code-end]

---