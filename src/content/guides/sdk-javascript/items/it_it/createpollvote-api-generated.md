Registra un voto su un sondaggio, o sposta uno esistente su un’opzione diversa. Un elettore ha al massimo un voto per sondaggio, quindi chiamare nuovamente questo endpoint per lo stesso elettore sposta il suo voto invece di aggiungerne uno.

Questo rispetta le impostazioni del sondaggio del sito: se il voto è impostato solo per utenti autenticati, un voto con solo anonUserId viene rifiutato, e i voti anonimi sono limitati per IP per sondaggio.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Risposta

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId è opzionale e omesso qui
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]