Прикрепите опрос к комментарию или задайте полное состояние уже существующего опроса.

Опции сопоставляются по id: опция, отправленная с id существующей опции, сохраняет свои голоса (и получает новый ярлык и позицию), опция без id добавляется, а существующие опции, отсутствующие в списке, удаляются вместе с отданными за них голосами.

Отсутствие id существующих опций в опросе, в котором уже есть голоса, удаляет все опции, поэтому необходимо установить replaceVotes=true.

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Ответ

Возвращает: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const commentId: string = "9876543210";

const optionA: CommentPollOptionInput = { text: "Dark mode" };
const optionB: CommentPollOptionInput = { text: "Light mode" };

const pollInput: CommentPollPutInput = {
  question: "Which UI theme do you prefer?",
  options: [optionA, optionB],
};

const replaceVotes: boolean = true;

const result: SavePollResponse = await putPoll(tenantId, commentId, pollInput, replaceVotes);
[inline-code-end]