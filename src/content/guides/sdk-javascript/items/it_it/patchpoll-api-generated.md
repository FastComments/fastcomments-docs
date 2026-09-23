Modifica un sondaggio in loco, mantenendo i suoi risultati: cambia la domanda, rinomina un'opzione, chiudi o riapri il sondaggio,
o modifica chi può vedere i votanti. Le opzioni sono identificate tramite id - per aggiungerle, rimuoverle o riordinarle, esegui un PUT dell'elenco completo.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| commentPollPatch | CommentPollPatch | Sì |  |

## Risposta

Restituisce: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio patchPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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