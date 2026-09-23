Les votes individuels derrière le décompte d'un sondage, du plus ancien au plus récent.

Un sondage appartient à un commentaire, donc les votes sont toujours lus un sondage à la fois - commentId est requis. Cela maintient chaque requête sur les index déjà présents dans la collection.

Respecte la confidentialité du sondage : les votes d'un sondage anonyme ne peuvent pas être lus (poll-anonymous), ici ou par identifiant.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemple getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]

---