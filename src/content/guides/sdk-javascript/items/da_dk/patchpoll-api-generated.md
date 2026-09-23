Rediger en afstemning på stedet, mens du bevarer stemmetal: ændr spørgsmålet, omdøb en mulighed, luk eller genåbn den, eller ændr hvem der kan se vælgerne. Muligheder adresseres efter id - for at tilføje, fjerne eller omarrangere dem, PUT den fulde liste.

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| commentPollPatch | CommentPollPatch | Ja |  |

## Response

Returnerer: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'patchPoll Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_9876543210";

  const pollPatch: CommentPollPatch = {
    question: "Which new feature would you like to see?",
    options: [
      { id: "opt-1", label: "Dark Mode" },
      { id: "opt-2", label: "Multi-language Support" }
    ]
  };

  const response: SavePollResponse = await patchPoll(tenantId, commentId, pollPatch);
  console.log(response);
})();
[inline-code-end]

---