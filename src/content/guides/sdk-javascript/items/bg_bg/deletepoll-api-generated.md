Премахнете анкета от нейния коментар, заедно с всички гласове, дадени за нея. Самият коментар остава непроменен.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за deletePoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePoll(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const commentId: string = "comment_a1b2c3d4";
  const result: APIEmptyResponse = await deletePoll(tenantId, commentId);
  console.log(result);
}
[inline-code-end]