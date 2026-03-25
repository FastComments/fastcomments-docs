## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Sim |  |

## Resposta

Retorna: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "u_987654",
  planId: "pro_monthly",
  startDate: new Date().toISOString(),
  trialDays: 14, // parâmetro opcional demonstrado
  metadata: { source: "marketing-email" } // parâmetro opcional demonstrado
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
const subscription: APIUserSubscription = result.subscription;
[inline-code-end]