A `PollVote` é a resposta de uma pessoa a uma enquete. As contagens mostradas na própria enquete são mantidas em sincronia com estas, portanto você só precisa delas quando quiser saber *quem* votou em quê, em vez dos totais.

Um eleitor tem no máximo um voto por enquete. Votar novamente move o voto existente para a nova opção em vez de adicionar um segundo, e `updatedAt` registra quando isso aconteceu.

`voterId` é o `userId` quando o eleitor estava logado, e o `anonUserId` caso contrário.

[inline-code-attrs-start title = 'Estrutura PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** The userId when the voter was logged in, otherwise the anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** When the voter last moved their vote to a different option. **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

A configuração `privacy` da enquete se aplica a esta API da mesma forma que se aplica no widget de comentários:

- **Anonymous** (the default): nobody can see how anyone voted, so the votes cannot be read.
  `GET /api/v1/poll-votes` and `GET /api/v1/poll-votes/:id` respond with `poll-anonymous`. The poll's counts
  are still available from `GET /api/v1/polls/:commentId`.
- **Admins and moderators**: your API key belongs to your site's admin, so it can read the votes.
- **Everyone**: the votes can be read.

Poll privacy can be narrowed but not widened once it has votes.