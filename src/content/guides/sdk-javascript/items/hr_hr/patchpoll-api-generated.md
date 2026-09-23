Uredite anketu na licu mjesta, zadržavajući njene rezultate: promijenite pitanje, preimenujte opciju, zatvorite ili ponovno otvorite,
ili promijenite tko može vidjeti glasače. Opcije se adresiraju po id-u – za dodavanje, uklanjanje ili promjenu redoslijeda, PUT cijeli popis.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| commentPollPatch | CommentPollPatch | Da |  |

## Odgovor

Vraća: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Primjer

[inline-code-attrs-start title = 'patchPoll Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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