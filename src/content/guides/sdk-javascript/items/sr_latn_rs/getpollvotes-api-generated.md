Individualni glasovi iza zbirnih rezultata jedne ankete, najstariji prvi.

Anketa pripada komentaru, tako da se glasovi uvek čitaju po jednoj anketi odjednom – commentId je obavezan. To održava svaki upit na indeksima koje kolekcija već ima.

Poštuje privatnost ankete: glasovi anonimne ankete ne mogu biti pročitani (poll-anonymous), ni ovde ni po ID-u.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Odgovor

Vraća: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

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