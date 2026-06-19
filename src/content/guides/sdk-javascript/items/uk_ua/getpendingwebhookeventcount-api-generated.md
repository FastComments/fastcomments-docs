## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Ні |  |
| externalId | string | Ні |  |
| eventType | string | Ні |  |
| type | string | Ні |  |
| domain | string | Ні |  |
| attemptCountGT | number | Ні |  |

## Відповідь

Повертає: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'd5c7e8b4-3a1f-4b2e-9f6c-12ab34cd56ef';
  const commentId: string = 'cmt_000842';
  const externalId: string = 'post-77f4';
  const eventType: string = 'comment.updated';
  const typeParam: string = 'delivery';
  const domain: string = 'myblog.example.net';
  const attemptCountGT: number = 2;

  const result: GetPendingWebhookEventCountResponse = await getPendingWebhookEventCount(
    tenantId,
    commentId,
    externalId,
    eventType,
    typeParam,
    domain,
    attemptCountGT
  );

  console.log(result);
})();
[inline-code-end]

---