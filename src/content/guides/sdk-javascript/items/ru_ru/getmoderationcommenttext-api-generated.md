## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| tenantId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getModerationCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Вызов только с обязательным параметром
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Вызов с необязательными параметрами
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]