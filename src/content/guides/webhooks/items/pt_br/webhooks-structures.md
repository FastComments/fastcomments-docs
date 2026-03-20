A única estrutura enviada via webhooks é o objeto WebhookComment, descrito em TypeScript abaixo.

#### Estrutura do Objeto WebhookComment

##### Estrutura do evento "create"
O corpo da requisição do evento "create" é um objeto WebhookComment.

##### Estrutura do evento "update"
O corpo da requisição do evento "update" é um objeto WebhookComment.

##### Estrutura do evento "delete"
O corpo da requisição do evento "delete" é um objeto WebhookComment.

    Alteração a partir de 14 de novembro de 2023
    Anteriormente, o corpo da requisição do evento "delete" continha apenas o id do comentário. Agora ele contém o comentário completo no momento da exclusão.


[inline-code-attrs-start title = 'O Objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** O id do comentário. **/
    id: string
    /** O id ou URL que identifica o tópico de comentários. Normalizado. **/
    urlId: string
    /** A URL que aponta para onde o comentário foi feito. **/
    url?: string
    /** O id do usuário que deixou o comentário. Se SSO, prefixado com o id do tenant. **/
    userId?: string
    /** O email do usuário que deixou o comentário. **/
    commenterEmail?: string
    /** O nome do usuário exibido no widget de comentários. Com SSO, pode ser displayName. **/
    commenterName: string
    /** Texto bruto do comentário. **/
    comment: string
    /** Texto do comentário após o parsing. **/
    commentHTML: string
    /** Id externo do comentário. **/
    externalId?: string
    /** O id do comentário pai. **/
    parentId?: string | null
    /** A data em UTC quando o comentário foi feito. **/
    date: UTC_ISO_DateString
    /** Karma combinado (positivos - negativos) dos votos. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Verdadeiro se o usuário estava logado quando comentou, ou se ele verificou o comentário, ou se verificou a sessão quando o comentário foi feito. **/
    verified: boolean
    /** Data em que o comentário foi verificado. **/
    verifiedDate?: number
    /** Se um moderador marcou o comentário como revisado. **/
    reviewed: boolean
    /** A localização, ou codificação base64, do avatar. Será base64 apenas se esse foi o valor passado com SSO. **/
    avatarSrc?: string
    /** O comentário foi marcado manualmente ou automaticamente como spam? **/
    isSpam: boolean
    /** O comentário foi marcado automaticamente como spam? **/
    aiDeterminedSpam: boolean
    /** Existem imagens no comentário? **/
    hasImages: boolean
    /** O número da página em que o comentário está para a ordenação "Most Relevant". **/
    pageNumber: number
    /** O número da página em que o comentário está para a ordenação "Oldest First". **/
    pageNumberOF: number
    /** O número da página em que o comentário está para a ordenação "Newest First". **/
    pageNumberNF: number
    /** O comentário foi aprovado automaticamente ou manualmente? **/
    approved: boolean
    /** O código de localidade (formato: en_us) do usuário quando o comentário foi escrito. **/
    locale: string
    /** As @mentions escritas no comentário que foram parseadas com sucesso. **/
    mentions?: CommentUserMention[]
    /** O domínio de onde o comentário vem. **/
    domain?: string
    /** A lista opcional de ids de grupo de moderação associados a este comentário. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Quando usuários são marcados em um comentário, as informações são armazenadas em uma lista chamada `mentions`. Cada objeto nessa lista
tem a seguinte estrutura.

[inline-code-attrs-start title = 'O Objeto de Menções do Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** O id do usuário. Para usuários SSO, isso terá o id do seu tenant prefixado. **/
    id: string
    /** O texto final da tag @mention, incluindo o símbolo @. **/
    tag: string
    /** O texto original da tag @mention, incluindo o símbolo @. **/
    rawTag: string
    /** Que tipo de usuário foi marcado. user = conta FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se o usuário optar por não receber notificações, isto ainda será definido como true. **/
    sent: boolean
}
[inline-code-end]

#### Métodos HTTP

Você pode configurar o método HTTP para cada tipo de evento do webhook no painel de administração:

- **Evento "create"**: POST ou PUT (padrão: PUT)
- **Evento "update"**: POST ou PUT (padrão: PUT)
- **Evento "delete"**: DELETE, POST ou PUT (padrão: DELETE)

Como todas as requisições contêm um ID, as operações Create e Update são idempotentes por padrão (PUT). Repetir a mesma requisição de Create ou Update não deve criar objetos duplicados do seu lado.

#### Cabeçalhos da Requisição

Cada requisição do webhook inclui os seguintes cabeçalhos:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Seu segredo de API |
| `X-FastComments-Timestamp` | Timestamp Unix (segundos) quando a requisição foi assinada |
| `X-FastComments-Signature` | Assinatura HMAC-SHA256 (`sha256=<hex>`) |

Veja [Segurança e Tokens de API](/guide-webhooks.html#webhooks-api-tokens) para informações sobre como verificar a assinatura HMAC.