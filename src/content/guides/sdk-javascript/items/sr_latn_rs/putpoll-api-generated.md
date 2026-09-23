Prikačite anketu na komentar, ili postavite kompletno stanje ankete koju već ima.

Opcije se podudaraju po id‑u: opcija poslata sa id‑jem postojeće opcije zadržava svoje glasove (i preuzima novi naziv i poziciju), opcija poslata bez id‑ja se dodaje, a postojeće opcije izostavljene iz liste se uklanjaju zajedno sa glasovima koji su na njih dati.

Ako se ne zadrže postojeći id‑jevi opcija u anketi koja ima glasove, svi se brišu, pa je potrebno postaviti replaceVotes=true.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Odgovor

Vraća: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Primer

[inline-code-attrs-start title = 'putPoll Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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