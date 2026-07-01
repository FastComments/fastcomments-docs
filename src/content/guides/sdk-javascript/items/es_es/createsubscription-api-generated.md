## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Sí |  |

## Respuesta

Devuelve: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";

    const subscriptionData: CreateAPIUserSubscriptionData = {
        userId: "user-456",
        planId: "premium-monthly",
        startDate: new Date(),
        trialPeriodDays: 14 // campo opcional
    };

    const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, subscriptionData);
    console.log(result);
})();
[inline-code-end]