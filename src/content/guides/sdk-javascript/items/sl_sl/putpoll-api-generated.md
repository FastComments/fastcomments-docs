Pripnite anketo k komentarju ali nastavite celotno stanje ankete, ki jo že ima.

Možnosti se ujemajo po id: možnost, poslana z id obstoječe možnosti, obdrži svoje glasove (in prevzame novo oznako ter položaj), možnost brez id se doda, obstoječe možnosti, ki jih ni na seznamu, pa se odstranijo skupaj z glasovi, oddanimi zanje.

Če na anketi z glasovi ne obdržite nobenega id obstoječe možnosti, se vsi glasovi izbrišejo, zato je potreben replaceVotes=true.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| commentPollPutInput | CommentPollPutInput | Da |  |
| replaceVotes | boolean | Ne |  |

## Odgovor

Vrne: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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