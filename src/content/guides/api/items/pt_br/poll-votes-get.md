[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Lista os votos individuais por trás das contagens de uma enquete, do mais antigo ao mais recente. Um crédito por cada 100 votos retornados.

Uma enquete pertence a um comentário, portanto os votos são lidos uma enquete por vez e `commentId` é obrigatório. Refine ainda mais
com `voterId` para verificar como uma pessoa votou, ou com `optionId` para listar todos que escolheram uma determinada opção.

No máximo 1000 votos são retornados por chamada. Use `skip` para paginar mais resultados.

A configuração `privacy` da enquete é respeitada: os votos em uma enquete anônima não podem ser lidos, e a requisição falha
com `poll-anonymous`. Veja a estrutura `PollVote` para detalhes.

[inline-code-attrs-start title = 'Exemplo cURL de PollVotes Get'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição Get de PollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta Get de PollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Contando Votos Por Opção

Você não precisa somar esses votos para obter os resultados - a enquete já possui suas próprias contagens. Leia a enquete com
`GET /api/v1/polls/:commentId` em vez disso, e use esta API quando precisar saber quem votou.

### Cada Enquete em uma Página

Não há listagem de votos em nível de página. Para relatar sobre uma página inteira, recupere seus comentários com
`GET /api/v1/comments`, que retorna a enquete de cada comentário e suas contagens, e então leia os votos das
enquetes que lhe interessam.