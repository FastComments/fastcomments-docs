---
Allega un sondaggio a un commento, o imposta lo stato completo del sondaggio che ha già.

Le opzioni sono abbinate per id: un'opzione inviata con l'id di un'opzione esistente mantiene i suoi voti (e prende la nuova etichetta e posizione), un'opzione inviata senza un id viene aggiunta, e le opzioni esistenti omesse dalla lista vengono rimosse insieme ai voti espressi su di esse.

Mantenere nessun id di opzione esistente su un sondaggio che ha voti elimina tutti, quindi è necessario replaceVotes=true.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'Esempio putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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