[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Edita uma enquete sem perturbar seus votos. Use isso para corrigir um erro de digitação na pergunta ou em uma opção, para fechar ou reabrir a enquete, ou para mudar quem pode ver quem votou.

As opções são referenciadas pelo seu `id`, e um `PATCH` renomeia as que você especificar. Para adicionar, remover ou reordenar opções, envie a lista completa de opções para `PUT /api/v1/polls/:commentId`: as opções que você enviar com seus ids mantêm seus votos também.

Cada campo é opcional, mas ao menos um deve ser fornecido.

[inline-code-attrs-start title = 'Exemplo cURL de Patch de Enquete'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Exemplo cURL para Fechar uma Enquete Agora'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Patch de Enquete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Renomeia opções existentes. Cada id fornecido já deve estar na enquete. **/
    options?: { id: string, label: string }[] | null
    /** Uma data no passado fecha a enquete agora. null reabre uma enquete fechada. **/
    closesAt?: string | null
    /** 0 anônimo, 1 administradores e moderadores, 2 todos. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Patch de Enquete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Incluído em caso de falha. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Outras Observações

- Nomear um id de opção que não está na enquete falha com `poll-invalid` em vez de simplesmente não fazer nada.
- Os rótulos devem permanecer únicos dentro da enquete, contando as opções que você não está alterando.
- Ao contrário de criar uma enquete, `closesAt` pode estar no passado aqui – é assim que você fecha uma enquete imediatamente.
- A privacidade da enquete pode ser restringida, mas não ampliada, depois que ela tem votos.
- Um comentário bloqueado não pode ter sua enquete alterada, e falha com `locked`.

---