## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'postFlagComment Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";

  // Тільки обов’язкові параметри
  const result1: APIEmptyResponse = await postFlagComment(tenantId, commentId);

  // Включаючи необов’язкові параметри
  const broadcastId: string = "brd_54321";
  const sso: string = "user@example.com";
  const result2: APIEmptyResponse = await postFlagComment(tenantId, commentId, broadcastId, sso);

  console.log(result1, result2);
}
runExample();
[inline-code-end]