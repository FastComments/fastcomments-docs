Dołącz ankietę do komentarza lub ustaw pełny stan ankiety, którą już ma.

Opcje są dopasowywane po identyfikatorze: opcja wysłana z identyfikatorem istniejącej opcji zachowuje swoje głosy (i przyjmuje nową etykietę oraz pozycję), opcja wysłana bez identyfikatora jest dodawana, a istniejące opcje pominięte na liście są usuwane wraz z oddanymi na nie głosami.

Brak istniejących identyfikatorów opcji w ankiecie, która ma głosy, usuwa wszystkie, więc wymaga to replaceVotes=true.

## Parameters

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'Przykład putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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