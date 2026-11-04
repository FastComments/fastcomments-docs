Отдельные голоса, стоящие за итогами одного опроса, начиная с самых старых.

Опрос принадлежит комментарию, поэтому голоса всегда читаются по одному опросу за раз — commentId обязателен. Это
позволяет каждому запросу использовать уже существующие индексы коллекции.

Соблюдает настройки приватности опроса: голоса анонимного опроса нельзя прочитать (poll-anonymous), ни здесь, ни по id.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| voterId | string | Нет |  |
| optionId | string | Нет |  |
| skip | number | Нет |  |

## Ответ

Возвращает: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]