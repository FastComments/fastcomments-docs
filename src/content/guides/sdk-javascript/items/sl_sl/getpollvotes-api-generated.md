Posamezni glasovi za rezultate ene ankete, od najstarejšega naprej.

Anketa pripada komentarju, zato se glasovi vedno berejo eno anketo naenkrat - commentId je obvezen. To
ohrani vsako poizvedbo na indeksih, ki jih zbirka že ima.

Upošteva zasebnost ankete: glasov anonimne ankete ni mogoče brati (poll-anonymous), niti tukaj niti po id.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| voterId | string | Ne |  |
| optionId | string | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vrne: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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