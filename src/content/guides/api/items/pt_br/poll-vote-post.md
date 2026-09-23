[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Registra um voto em uma enquete.

Um eleitor tem no máximo um voto por enquete. Chamar este endpoint novamente para o mesmo eleitor move seu voto para a nova
opção em vez de adicionar um segundo, e votar na opção que ele já escolheu não faz nada.

A resposta inclui a enquete, portanto você obtém as contagens atualizadas sem uma segunda solicitação.

[inline-code-attrs-start title = 'Exemplo cURL de Criação de PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Exemplo cURL de Criação de PollVote Anônima'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Criação de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** One of userId or anonUserId is required. **/
    userId?: string
    anonUserId?: string
    /** The end user's IP, used for the anonymous rate limit. Defaults to the caller's IP. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Criação de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Votos Anônimos

Defina `anonUserId` em vez de `userId` para registrar um voto para alguém que não está conectado. Esse ID não precisa
corresponder a um usuário em nenhum lugar – ele apenas identifica a sessão, de modo que a mesma pessoa não seja contada duas vezes.

A votação anônima precisa estar habilitada para o seu site. Se a votação estiver limitada a usuários conectados, um voto com apenas um
`anonUserId` falha com `poll-login-required`.

Votos anônimos também são limitados por taxa por IP por enquete, para impedir que uma pessoa manipule a enquete limpando sua
sessão. Envie o `ip` do usuário final para que o limite se aplique a ele em vez de ao seu servidor.

### Outras Observações

- Um `userId` deve ser um usuário que exista no seu site. Votos para um usuário pertencente a outro site são rejeitados.
- Votar em uma enquete fechada falha com `poll-closed`.
- Esta API atualiza as contagens na enquete e as envia para os widgets conectados em tempo real.

---