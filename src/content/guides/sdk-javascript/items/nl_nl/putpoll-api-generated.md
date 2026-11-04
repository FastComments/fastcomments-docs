Voeg een poll toe aan een comment, of stel de volledige status in van de poll die deze al heeft.

Opties worden gematcht op id: een optie die wordt verzonden met de id van een bestaande optie behoudt zijn stemmen (en krijgt het
nieuwe label en de nieuwe positie), een optie zonder id wordt toegevoegd, en bestaande opties die niet in de lijst staan
worden verwijderd, samen met de op hen uitgebrachte stemmen.

Als je geen bestaande optie-ids behoudt op een poll waarop stemmen zijn uitgebracht, worden alle stemmen verwijderd, dus daarvoor is replaceVotes=true nodig.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Response

Geeft terug: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'putPoll Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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