Modifiez un sondage sur place, en conservant ses résultats : changez la question, renommez une option, fermez‑ou rouvrez‑le, ou modifiez qui peut voir les votants. Les options sont identifiées par leur id — pour les ajouter, les supprimer ou les réordonner, effectuez un PUT de la liste complète.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| commentPollPatch | CommentPollPatch | Oui |  |

## Réponse

Retourne : [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de patchPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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