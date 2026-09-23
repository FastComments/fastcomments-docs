Vedhæft en afstemning til en kommentar, eller indstil den fulde tilstand af den afstemning, den allerede har.

Muligheder matches efter id: en mulighed sendt med id'et på en eksisterende mulighed bevarer sine stemmer (og får den nye etiket og position), en mulighed sendt uden et id tilføjes, og eksisterende muligheder, der udelades fra listen, fjernes sammen med de afgivne stemmer.

Hvis ingen eksisterende mulighedsid'er bevares på en afstemning, der har stemmer, slettes alle, så det kræver replaceVotes=true.

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| commentPollPutInput | CommentPollPutInput | Ja |  |
| replaceVotes | boolean | Nej |  |

## Svar

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'putPoll Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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