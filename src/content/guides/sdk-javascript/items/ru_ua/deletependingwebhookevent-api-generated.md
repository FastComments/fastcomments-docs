## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'deletePendingWebhookEvent Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42f7c9b1';
  const id: string = 'pending_webhook_ev_8f3b9a2d';
  const reason?: string = undefined; // пример необязательного параметра (не обязателен для функции)
  const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
  console.log(result);
})();
[inline-code-end]

---