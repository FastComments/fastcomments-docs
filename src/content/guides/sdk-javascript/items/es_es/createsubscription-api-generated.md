## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Sí |  |

## Respuesta

Devuelve: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "u_987654",
  planId: "pro_monthly",
  startDate: new Date().toISOString(),
  trialDays: 14, // parámetro opcional demostrado
  metadata: { source: "marketing-email" } // parámetro opcional demostrado
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
const subscription: APIUserSubscription = result.subscription;
[inline-code-end]

---