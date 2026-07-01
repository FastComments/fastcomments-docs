## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Yes |  |

## Réponse

Retourne : [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Exemple

[inline-code-attrs-start title = 'createSubscription Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";

    const subscriptionData: CreateAPIUserSubscriptionData = {
        userId: "user-456",
        planId: "premium-monthly",
        startDate: new Date(),
        trialPeriodDays: 14 // champ optionnel
    };

    const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, subscriptionData);
    console.log(result);
})();
[inline-code-end]