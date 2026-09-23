Enregistrez un vote sur un sondage, ou déplacez un vote existant vers une autre option. Un électeur ne peut avoir qu’un seul vote par sondage, donc appeler cette fonction à nouveau pour le même électeur déplace son vote plutôt que d’en ajouter un.

Cela respecte les paramètres de sondage du site : si le vote est limité aux utilisateurs connectés, un vote avec uniquement un anonUserId est rejeté, et les votes anonymes sont limités en fréquence par IP et par sondage.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createPollVoteBody | CreatePollVoteBody | Yes |  |

## Réponse

Retourne : [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId est optionnel et omis ici
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]