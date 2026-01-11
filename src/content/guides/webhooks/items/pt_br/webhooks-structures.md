A única estrutura enviada via webhooks é o objeto WebhookComment, descrito em TypeScript abaixo.

#### A Estrutura do Objeto WebhookComment

##### A Estrutura do Evento "Create"
O corpo da requisição do evento "create" é um objeto WebhookComment.

##### A Estrutura do Evento "Update"
O corpo da requisição do evento "update" é um objeto WebhookComment.

##### A Estrutura do Evento "Delete"
O corpo da requisição do evento "delete" é um objeto WebhookComment.

    Mudança a partir de 14 de nov de 2023
    Anteriormente, o corpo da requisição do evento "delete" continha apenas o id do comentário. Agora contém o comentário completo no momento da exclusão.


[inline-code-attrs-start title = 'O Objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** O id do comentário. **/
    id: string
    /** O id ou URL que identifica o thread do comentário. Normalizado. **/
    urlId: string
    /** A URL que aponta para onde o comentário foi deixado. **/
    url?: string
    /** O id do usuário que deixou o comentário. Se SSO, prefixado com tenant id. **/
    userId?: string
    /** O email do usuário que deixou o comentário. **/
    commenterEmail?: string
    /** O nome do usuário que aparece no widget de comentário. Com SSO, pode ser displayName. **/
    commenterName: string
    /** Texto bruto do comentário. **/
    comment: string
    /** Texto do comentário após o parsing. **/
    commentHTML: string
    /** Id externo do comentário. **/
    externalId?: string
    /** O id do comentário pai. **/
    parentId?: string | null
    /** A data UTC quando o comentário foi deixado. **/
    date: UTC_ISO_DateString
    /** Karma combinado (up - down) dos votos. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True se o usuário estava logado quando comentou, ou se verificou o comentário, ou se verificou sua sessão quando o comentário foi deixado. **/
    verified: boolean
    /** Data quando o comentário foi verificado. **/
    verifiedDate?: number
    /** Se um moderador marcou o comentário como revisado. **/
    reviewed: boolean
    /** A localização, ou codificação base64, do avatar. Será base64 somente se esse foi o valor passado com SSO. **/
    avatarSrc?: string
    /** O comentário foi marcado manualmente ou automaticamente como spam? **/
    isSpam: boolean
    /** O comentário foi marcado automaticamente como spam? **/
    aiDeterminedSpam: boolean
    /** Existem imagens no comentário? **/
    hasImages: boolean
    /** O número da página em que o comentário está para a direção de ordenação "Most Relevant". **/
    pageNumber: number
    /** O número da página em que o comentário está para a direção de ordenação "Oldest First". **/
    pageNumberOF: number
    /** O número da página em que o comentário está para a direção de ordenação "Newest First". **/
    pageNumberNF: number
    /** O comentário foi aprovado automaticamente ou manualmente? **/
    approved: boolean
    /** O código de localidade (formato: en_us) do usuário quando o comentário foi escrito. **/
    locale: string
    /** As @menções escritas no comentário que foram parseadas com sucesso. **/
    mentions?: CommentUserMention[]
    /** O domínio de onde o comentário veio. **/
    domain?: string
    /** A lista opcional de ids de grupos de moderação associados a este comentário. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Quando usuários são marcados em um comentário, a informação é armazenada em uma lista chamada `mentions`. Cada objeto nessa lista
tem a seguinte estrutura.

[inline-code-attrs-start title = 'O Objeto de Menções do Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** O id do usuário. Para usuários SSO, isso terá seu tenant id prefixado. **/
    id: string
    /** O texto final da @menção, incluindo o símbolo @. **/
    tag: string
    /** O texto original da @menção, incluindo o símbolo @. **/
    rawTag: string
    /** Que tipo de usuário foi marcado. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se o usuário optar por não receber notificações, isto ainda será definido como true. **/
    sent: boolean
}
[inline-code-end]

#### Métodos HTTP

Você pode configurar o método HTTP para cada tipo de evento do webhook no painel de administração:

- **Evento de criação**: POST ou PUT (padrão: PUT)
- **Evento de atualização**: POST ou PUT (padrão: PUT)
- **Evento de exclusão**: DELETE, POST ou PUT (padrão: DELETE)

Como todas as requisições contêm um ID, operações de Create e Update são idempotentes por padrão (PUT). Repetir a mesma requisição de Create ou Update não deve criar objetos duplicados do seu lado.

#### Cabeçalhos da Requisição

Cada requisição de webhook inclui os seguintes cabeçalhos:

| Header | Descrição |
|--------|-----------|
| `Content-Type` | `application/json` |
| `token` | Seu segredo de API |
| `X-FastComments-Timestamp` | Timestamp Unix (segundos) quando a requisição foi assinada |
| `X-FastComments-Signature` | Assinatura HMAC-SHA256 (`sha256=<hex>`) |

Veja [Security & API Tokens](/guides/webhooks/webhooks-api-tokens) para informações sobre verificação da assinatura HMAC.