Zabeleži glas na anketi ali premakni obstoječi glas na drugo možnost. Glasovalec ima največ en glas na anketo, zato ponovni klic za istega glasovalca premakne njihov glas namesto da bi dodal novega.

To upošteva nastavitve ankete na spletnem mestu: če je glasovanje nastavljeno le za prijavljene uporabnike, je glas z le anonUserId zavrnjen, anonimni glasovi pa so omejeni po IP-ju na anketo.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createPollVoteBody | CreatePollVoteBody | Da |  |

## Odgovor

Vrne: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId je neobvezen in je tukaj izpuščen
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]