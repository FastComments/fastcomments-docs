---
Zabilježite glas na anketi ili premjestite postojeći glas na drugu opciju. Glasač ima najviše jedan glas po anketi, pa ponovno pozivanje ove funkcije za istog glasača premješta njihov glas umjesto da doda novi.

Ovo poštuje postavke ankete na stranici: ako je glasanje omogućeno samo za prijavljene korisnike, glas s samo anonUserId-om se odbija, a anonimni glasovi su ograničeni po IP-u po anketi.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Response

Vraća: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'Primjer createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId je opcionalan i ovdje je izostavljen
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]

---