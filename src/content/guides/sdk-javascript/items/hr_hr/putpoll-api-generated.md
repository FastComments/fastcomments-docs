Priložite anketu uz komentar ili postavite cijelo stanje ankete koju već ima.

Opcije se usklađuju po id: opcija poslana s id postojeće opcije zadržava svoje glasove (i preuzima novu oznaku i poziciju), opcija poslana bez id dodaje se, a postojeće opcije izostavljene s popisa uklanjaju se zajedno s glasovima danima na njima.

Ako se na anketi koja ima glasove ne zadrži nijedan id postojeće opcije, svi se glasovi brišu, pa je za to potrebno replaceVotes=true.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| commentPollPutInput | CommentPollPutInput | Da |  |
| replaceVotes | boolean | Ne |  |

## Odgovor

Vraća: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Primjer

[inline-code-attrs-start title = 'putPoll primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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