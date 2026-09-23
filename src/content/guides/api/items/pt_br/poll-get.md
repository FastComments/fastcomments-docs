[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Lê a enquete anexada a um comentário, com suas contagens de votos atuais.

Enquetes também são retornadas no próprio comentário pelas APIs de comentário, portanto use isto quando você quiser apenas os resultados e não o comentário completo.

[inline-code-attrs-start title = 'Exemplo cURL de Obtenção de Enquete'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Obtenção de Enquete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Obtenção de Enquete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Um comentário que não tem enquete, um comentário que foi excluído e um ID de comentário que não existe respondem da mesma forma, com `poll-not-found`.