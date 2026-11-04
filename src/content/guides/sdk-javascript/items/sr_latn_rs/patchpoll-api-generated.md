Izmenite anketu na licu mesta, zadržavajući njene rezultate: promenite pitanje, preimenujte opciju, zatvorite je ili je ponovo otvorite,
ili promenite ko može da vidi glasače. Opcijama se adresira preko id - da biste ih dodali, uklonili ili preuredili, pošaljite PUT sa celom listom.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| commentPollPatch | CommentPollPatch | Da |  |

## Odgovor

Vraća: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Primer

[inline-code-attrs-start title = 'patchPoll Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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