Прикачите анкету коментару, или поставите пун статус анкете коју већ има.

Опције се упоређују по id-у: опција послата са id-ом постојеће опције задржава своје гласове (и преузима
нову ознаку и позицију), опција послата без id-а се додаје, а постојеће опције изостављене из листе
се уклањају заједно са гласовима датим за њих.

Остављање без постојећих id-ова опција у анкети која има гласове брише све њих, па је потребно replaceVotes=true.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Одговор

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Пример

[inline-code-attrs-start title = 'putPoll пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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