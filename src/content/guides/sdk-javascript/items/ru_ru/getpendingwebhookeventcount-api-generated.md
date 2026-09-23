## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Нет |  |
| externalId | string | Нет |  |
| eventType | string | Нет |  |
| type | string | Нет |  |
| domain | string | Нет |  |
| attemptCountGT | number | Нет |  |

## Ответ

Возвращает: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_42";
  const commentId: string = "comment_1001";
  const eventType: string = "comment.deleted";
  const domain: string = "myblog.com";

  const result: GetPendingWebhookEventCountResponse = await getPendingWebhookEventCount(
    tenantId,
    commentId,
    undefined,
    eventType,
    undefined,
    domain
  );

  console.log(result);
}

runExample();
[inline-code-end]