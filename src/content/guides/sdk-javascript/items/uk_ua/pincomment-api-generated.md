## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| broadcastId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`PinCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinCommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'pinComment Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant-001';
  const commentId: string = 'comment-5678';
  const broadcastId: string = 'broadcast-2023';
  const ssoToken: string = 'sso-xyz-789';

  const pinResult: PinCommentResponse = await pinComment(tenantId, commentId, broadcastId);
  const pinResultWithSso: PinCommentResponse = await pinComment(tenantId, commentId, broadcastId, ssoToken);
})();
[inline-code-end]