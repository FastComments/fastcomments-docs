Un `PollVote` est la réponse d’une personne à un sondage. Les décomptes affichés sur le sondage lui‑même sont maintenus à jour avec ceux‑ci, de sorte que vous n’avez besoin de ces informations que lorsque vous souhaitez savoir *qui* a voté pour quoi, plutôt que les totaux.

Un électeur ne peut avoir qu’un seul vote par sondage. Voter à nouveau déplace son vote existant vers la nouvelle option au lieu d’en ajouter un second, et `updatedAt` indique quand cela s’est produit.

`voterId` est le `userId` lorsque l’électeur était connecté, et le `anonUserId` sinon.

[inline-code-attrs-start title = 'Structure de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** L'userId lorsque l'électeur était connecté, sinon l'anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Quand l'électeur a déplacé son vote pour la dernière fois vers une option différente. **/
    updatedAt?: string
}
[inline-code-end]

### Confidentialité

Le paramètre `privacy` du sondage s’applique à cette API de la même manière qu’il s’applique dans le widget de commentaires :

- **Anonymous** (par défaut) : personne ne peut voir comment quiconque a voté, donc les votes ne peuvent pas être lus. `GET /api/v1/poll-votes` et `GET /api/v1/poll-votes/:id` répondent avec `poll-anonymous`. Les décomptes du sondage sont toujours disponibles via `GET /api/v1/polls/:commentId`.
- **Admins et modérateurs** : votre clé API appartient à l’administrateur de votre site, elle peut donc lire les votes.
- **Tout le monde** : les votes peuvent être lus.

La confidentialité du sondage peut être restreinte mais pas élargie une fois qu’il possède des votes.