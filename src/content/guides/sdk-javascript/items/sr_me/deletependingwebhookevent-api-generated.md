## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Примјер

[inline-code-attrs-start title = 'deletePendingWebhookEvent Примјер'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42f7c9b1';
  const id: string = 'pending_webhook_ev_8f3b9a2d';
  const reason?: string = undefined; // примјер опционалног параметра (није обавезно за функцију)
  const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
  console.log(result);
})();
[inline-code-end]

---