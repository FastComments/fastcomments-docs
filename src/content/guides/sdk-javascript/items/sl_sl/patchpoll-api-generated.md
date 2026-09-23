Uredite anketo na mestu, ohranite njene rezultate: spremenite vprašanje, preimenujte možnost, zaprite ali ponovno odprite jo,
ali spremenite, kdo lahko vidi glasovalce. Možnosti so naslovljene po id - za dodajanje, odstranjevanje ali preurejanje, pošljite celoten seznam s PUT.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| commentPollPatch | CommentPollPatch | Da |  |

## Odgovor

Vrne: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

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