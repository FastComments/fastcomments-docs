The individual votes behind one poll's tallies, oldest first.

Отдельные голоса, стоящие за подсчетами одного опроса, от старых к новым.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

Опрос принадлежит комментарию, поэтому голоса всегда читаются по одному опросу за раз — требуется commentId. Это сохраняет каждый запрос в пределах индексов, которые уже есть в коллекции.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

Соблюдает конфиденциальность опроса: голоса анонимного опроса не могут быть прочитаны (poll-anonymous), ни здесь, ни по идентификатору.

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Example

[inline-code-attrs-start title = 'getPollVotes Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---