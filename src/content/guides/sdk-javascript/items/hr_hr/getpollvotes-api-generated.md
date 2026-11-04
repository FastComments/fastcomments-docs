Pojedinačni glasovi iza zbroja jedne ankete, od najstarijih prema najnovijima.

Anketa pripada komentaru, pa se glasovi uvijek dohvaćaju za jednu anketu istovremeno - commentId je obavezan. To
osigurava da svaki upit koristi indekse koje kolekcija već ima.

Poštuje privatnost ankete: glasovi anonimne ankete ne mogu se čitati (poll-anonymous), ni ovdje ni po id-u.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| voterId | string | Ne |  |
| optionId | string | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getPollVotes primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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