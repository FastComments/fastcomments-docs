Прикріпити опитування до коментаря, або встановити повний стан вже існуючого опитування.

Опції збігаються за id: опція, надіслана з id існуючої опції, зберігає свої голоси (і отримує нову мітку та позицію), опція, надіслана без id, додається, а існуючі опції, які не включені у список, видаляються разом з голосами, відданими за них.

Якщо не залишати жодного існуючого id опції в опитуванні, яке має голоси, всі вони будуть видалені, тому потрібно встановити replaceVotes=true.

## Parameters

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'Приклад putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---