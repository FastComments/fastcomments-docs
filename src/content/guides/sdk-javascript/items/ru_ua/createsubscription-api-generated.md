## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Да |  |

## Ответ

Возвращает: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "user_98765",
  planId: "pro_monthly",
  paymentMethod: { type: "card", cardId: "card_abc123" },
  autoRenew: true,
  trialDays: 14, // необязательный параметр (продемонстрировано)
  metadata: { campaign: "spring_launch" } // необязательный параметр (продемонстрировано)
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
[inline-code-end]