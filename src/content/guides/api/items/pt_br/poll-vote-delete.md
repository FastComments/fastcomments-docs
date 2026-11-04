[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Cancela um voto. A opção em que o voto foi registrado recupera sua contagem, e o eleitor fica livre para votar novamente.

[inline-code-attrs-start title = 'Exemplo de cURL para exclusão de PollVote'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição de exclusão de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da resposta de exclusão de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Incluído em caso de falha. **/
    reason?: string
    /** A enquete com suas contagens atualizadas. **/
    poll: CommentPoll
}
[inline-code-end]

### Outras Observações

- Excluir o mesmo voto duas vezes responde com `not-found` na segunda vez, e as contagens permanecem inalteradas.
- Se a enquete foi substituída desde que o voto foi registrado, o voto é removido, mas nenhuma contagem é alterada, já que a
  substituição começou do zero.