Прикрепете анкета към коментар, или задайте пълното състояние на вече съществуващата анкета.

Опциите се съпоставят по id: опция, изпратена с id на съществуваща опция, запазва гласовете си (и получава новия етикет и позиция), опция, изпратена без id, се добавя, а съществуващите опции, пропуснати в списъка, се премахват заедно с гласовете, дадени за тях.

Ако не се запазят съществуващи id-та на опции в анкета, която има гласове, всички те се изтриват, затова е необходимо replaceVotes=true.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| commentPollPutInput | CommentPollPutInput | Да |  |
| replaceVotes | boolean | Не |  |

## Отговор

Връща: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Пример

[inline-code-attrs-start title = 'putPoll Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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