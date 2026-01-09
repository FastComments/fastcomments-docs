[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Esta rota fornece a habilidade de adicionar um único `Vote` autorizado. Votes podem ser `up` (+1) ou `down` (-1).

[inline-code-attrs-start title = 'Exemplo cURL de criação de Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Exemplo cURL de criação de Vote anônimo'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de requisição de criação de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de resposta de criação de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Incluído em caso de falha. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Criando Votos Anônimos

Votos anônimos podem ser criados definindo `anonUserId` nos query params em vez de `userId`.

Este id não precisa corresponder a um objeto de usuário em nenhum lugar (daí anônimo). É simplesmente um identificador
para a sessão, para que você possa buscar votos novamente na mesma sessão, para verificar se um comentário foi
votado.

Se você não tiver algo como "sessões anônimas" como a FastComments tem - você pode simplesmente
definir isso para um ID aleatório, como um UUID (embora apreciemos identificadores menores para economizar espaço).

### Outras Notas

- Esta API obedece às configurações em nível de tenant. Por exemplo, se você desativar a votação para uma determinada página, e tentar criar um voto via API, isso falhará com o código de erro `voting-disabled`.
- Esta API está ativa por padrão.
- Esta API atualizará os `votes` do `Comment` correspondente.

---