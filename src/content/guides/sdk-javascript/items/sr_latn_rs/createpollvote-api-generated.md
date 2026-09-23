Zapišite glas na anketi, ili premestite postojeći glas na drugu opciju. Glasač može imati najviše jedan glas po anketi, pa se ponovnim pozivanjem za istog glasača njegov glas pomera umesto da se doda novi.

Ovo poštuje podešavanja ankete na sajtu: ako je glasanje omogućeno samo za prijavljene korisnike, glas sa samo anonUserId se odbija, a anonimni glasovi su ograničeni po IP adresi po anketi.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Odgovor

Vraća: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Primer

[inline-code-attrs-start title = 'createPollVote Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId je opcionalan i ovde je izostavljen
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]