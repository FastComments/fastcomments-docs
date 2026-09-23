[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Anexa uma enquete a um comentário existente, ou define o estado completo da enquete que ele já possui.

O corpo é a enquete completa, e as opções que você envia se tornam as opções da enquete, nessa ordem. Cada opção é correspondida pelo seu `id`:

- Uma opção enviada com o `id` de uma opção existente mantém essa opção e seus votos. Seu rótulo e posição são atualizados para o que você enviou.
- Uma opção enviada sem um `id` é adicionada, sem votos.
- Uma opção existente que você omite é removida, juntamente com os votos lançados nela. `totalVotes` diminui na mesma quantidade.

Portanto, para adicionar uma opção, envie as opções atuais com seus ids mais a nova sem id. Para remover uma, envie a lista sem ela. Os ids das opções estão na enquete retornada por `GET /api/v1/polls/:commentId`.

Enviar nenhum id substitui todas as opções e exclui todos os votos já lançados na enquete. Se a enquete tem votos, isso requer `replaceVotes=true`, e sem isso a API responde com `replace-votes-required`.

Os outros campos também são substituídos: omitir `closesAt`, `privacy` ou `requireVoteToSeeResults` os redefine para o padrão. Para alterar um único campo e deixar o resto intacto, use `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Exemplo cURL de Enquete PUT'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Enquete PUT'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Obrigatório para manter nenhum dos ids de opção existentes quando a enquete tem votos, já que isso os exclui todos. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** O id de uma opção existente, para mantê-la e seus votos. Omitir para adicionar uma nova opção. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** A lista completa e ordenada. Opções existentes omitidas são removidas junto com seus votos. **/
    options: PollPutOption[]
    /** Deve estar no futuro quando o comentário ainda não tem enquete. Omitir para uma enquete que permanece aberta. **/
    closesAt?: string | null
    /** 0 anônimo (padrão), 1 administradores e moderadores, 2 todos. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Enquete PUT'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Incluído em caso de falha. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Outras Observações

- Um `id` que não está na enquete, ou o mesmo `id` fornecido duas vezes, falha com `poll-invalid`. Um comentário sem enquete ainda não tem ids de opção, portanto toda opção enviada para ele deve omitir `id`.
- A privacidade da enquete pode ser restringida, mas não ampliada, depois que ela tem votos.
- Esta API obedece às configurações do seu site. Se enquetes não estiverem habilitadas para o site ou página, falha com `polls-disabled`.
- Um comentário bloqueado não pode ter sua enquete alterada, e falha com `locked`.
- Widgets conectados são atualizados em tempo real, então os visualizadores veem a nova enquete sem recarregar.

---