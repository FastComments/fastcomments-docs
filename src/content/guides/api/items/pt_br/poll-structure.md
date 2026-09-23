Uma `Poll` está anexada a um comentário, em vez de ser um objeto independente. Ela é criada junto com o comentário (veja `POST /api/v1/comments`), ou adicionada a um comentário existente posteriormente com `PUT /api/v1/polls/:commentId`.

As contagens de votos são mantidas na própria enquete, portanto ler uma enquete fornece os resultados sem precisar somar nada. Os votos individuais por trás dessas contagens são objetos `PollVote`.

Cada opção tem um `id` que é gerado quando a enquete é criada. Esse id é usado para registrar um voto, renomear uma opção e manter uma opção (e seus votos) quando você `PUT` a enquete com opções adicionadas ou removidas. É a única forma segura de referir‑se a uma opção – nunca sua posição na lista.

[inline-code-attrs-start title = 'Estrutura da Enquete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

- Uma pergunta é obrigatória e tem no máximo 200 caracteres.
- Uma enquete tem entre 2 e 10 opções.
- Um rótulo de opção é obrigatório, tem no máximo 100 caracteres e deve ser exclusivo dentro da enquete (ignorando maiúsculas/minúsculas).
- `closesAt` deve estar no futuro quando a enquete é criada. Para fechar uma enquete imediatamente, faça `PATCH` com uma data no passado.

### Configurações do Site

Enquetes obedecem à configuração do seu site, que pode ser alterada em Personalizar Widget:

- As enquetes devem estar habilitadas antes que uma enquete possa ser criada, ou a API responderá com `polls-disabled`.
- A votação pode ser limitada a usuários autenticados, caso em que um voto enviado apenas com um `anonUserId` será rejeitado com `poll-login-required`.

---