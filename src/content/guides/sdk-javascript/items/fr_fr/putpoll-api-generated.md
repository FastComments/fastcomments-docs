Attachez un sondage à un commentaire, ou définissez l'état complet du sondage qu'il possède déjà.

Les options sont appariées par identifiant : une option envoyée avec l'identifiant d'une option existante conserve ses votes (et prend le nouveau libellé et la nouvelle position), une option envoyée sans identifiant est ajoutée, et les options existantes omises de la liste sont supprimées ainsi que les votes qui leur ont été attribués.

Ne conserver aucun identifiant d'option existant sur un sondage qui possède des votes supprime toutes les options, il faut donc que replaceVotes=true.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| commentPollPutInput | CommentPollPutInput | Oui |  |
| replaceVotes | boolean | Non |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemple putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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