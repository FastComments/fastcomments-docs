A `Poll` est attaché à un commentaire, plutôt que d'être un objet à part. Il est créé avec le commentaire  
(voir `POST /api/v1/comments`), ou ajouté à un commentaire existant plus tard avec `PUT /api/v1/polls/:commentId`.

Les comptes de votes sont conservés sur le sondage lui‑même, ainsi lire un sondage vous donne les résultats sans avoir à les additionner. Les votes individuels derrière ces comptes sont des objets `PollVote`.

Chaque option possède un `id` qui est généré lors de la création du sondage. Cet id est ce que vous utilisez pour voter, pour renommer une option, et pour conserver une option (et ses votes) lorsque vous `PUT` le sondage avec des options ajoutées ou supprimées. C’est la seule façon sûre de référencer une option – jamais sa position dans la liste.

[inline-code-attrs-start title = 'Structure du sondage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** When set and in the past, the poll is closed and no longer accepts votes. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. Absent means anonymous. **/
    privacy?: 0 | 1 | 2 | null
    /** When true, the counts are hidden from anyone who has not voted yet. Absent means false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limites

- Une question est requise, et ne dépasse pas 200 caractères.  
- Un sondage comporte entre 2 et 10 options.  
- Une étiquette d'option est requise, ne dépasse pas 100 caractères, et doit être unique au sein du sondage (sans tenir compte de la casse).  
- `closesAt` doit être dans le futur lors de la création du sondage. Pour fermer un sondage immédiatement, `PATCH`‑le avec une date dans le passé.

### Paramètres du site

Les sondages respectent la configuration de votre site, que vous pouvez modifier sous **Customize Widget** :

- Les sondages doivent être activés avant de pouvoir créer un sondage, sinon l'API répond avec `polls-disabled`.  
- Le vote peut être limité aux utilisateurs connectés, auquel cas un vote envoyé avec seulement un `anonUserId` est rejeté avec `poll-login-required`.