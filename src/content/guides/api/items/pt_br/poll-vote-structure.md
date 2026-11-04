A `PollVote` é a resposta de uma pessoa a uma enquete. As contagens exibidas na própria enquete são mantidas em sincronia com estas,
então você só precisa delas quando quiser saber *quem* votou em quê, em vez dos totais.

Um eleitor tem no máximo um voto por enquete. Votar novamente move o voto existente para a nova opção em vez de
adicionar um segundo, e `updatedAt` registra quando isso aconteceu.

`voterId` é o `userId` quando o eleitor estava logado, e o `anonUserId` caso contrário.

[inline-code-attrs-start title = 'Estrutura de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** O userId quando o eleitor estava logado, caso contrário o anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Quando o eleitor moveu seu voto pela última vez para uma opção diferente. **/
    updatedAt?: string
}
[inline-code-end]

### Privacidade

A configuração de `privacy` da enquete se aplica a esta API da mesma forma que se aplica no widget de comentários:

- **Anônima** (o padrão): ninguém pode ver como alguém votou, então os votos não podem ser lidos.
  `GET /api/v1/poll-votes` e `GET /api/v1/poll-votes/:id` respondem com `poll-anonymous`. As contagens da enquete
  ainda estão disponíveis em `GET /api/v1/polls/:commentId`.
- **Administradores e moderadores**: sua chave de API pertence ao administrador do seu site, então ela pode ler os votos.
- **Todos**: os votos podem ser lidos.

A privacidade da enquete pode ser restringida, mas não ampliada, depois que ela tiver votos.