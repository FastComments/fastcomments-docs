Записать голос в опросе или переместить существующий голос к другому варианту. У избирателя может быть не более одного голоса в опросе, поэтому повторный вызов для того же избирателя перемещает его голос, а не добавляет новый.

Это соблюдает настройки опросов сайта: если голосование разрешено только для авторизованных пользователей, голос с только anonUserId отклоняется, а анонимные голоса ограничиваются по частоте для каждого IP в каждом опросе.

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createPollVoteBody | CreatePollVoteBody | Да |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId является необязательным и здесь опущен
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]