## Параметры

| Название | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Yes |  |

## Ответ

Возвращает: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример createSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";

    const subscriptionData: CreateAPIUserSubscriptionData = {
        userId: "user-456",
        planId: "premium-monthly",
        startDate: new Date(),
        trialPeriodDays: 14 // необязательное поле
    };

    const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, subscriptionData);
    console.log(result);
})();
[inline-code-end]