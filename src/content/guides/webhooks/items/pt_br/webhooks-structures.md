A única estrutura enviada via webhooks é o objeto WebhookComment, descrito em TypeScript abaixo.

#### Estrutura do Objeto WebhookComment

##### Estrutura do Evento "Create"
O corpo da requisição do evento "create" é um objeto WebhookComment.

##### Estrutura do Evento "Update"
O corpo da requisição do evento "update" é um objeto WebhookComment.

##### Estrutura do Evento "Delete"
O corpo da requisição do evento "delete" é um objeto WebhookComment.

    Alteração a partir de 14 de novembro de 2023
    Anteriormente, o corpo da requisição do evento "delete" continha apenas o id do comentário. Agora contém o comentário completo no momento da deleção.


[inline-code-attrs-start title = 'O Objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** O id do comentário. **/
    id: string
    /** O id ou URL que identifica a thread de comentários. Normalizado. **/
    urlId: string
    /** A URL que aponta para o local onde o comentário foi deixado. **/
    url?: string
    /** O id do usuário que deixou o comentário. Se SSO, prefixado com tenant id. **/
    userId?: string
    /** O email do usuário que deixou o comentário. **/
    commenterEmail?: string
    /** O nome do usuário que aparece no widget de comentários. Com SSO, pode ser displayName. **/
    commenterName: string
    /** Texto bruto do comentário. **/
    comment: string
    /** Texto do comentário após o parsing. **/
    commentHTML: string
    /** Id externo do comentário. **/
    externalId?: string
    /** O id do comentário pai. **/
    parentId?: string | null
    /** A data UTC em que o comentário foi deixado. **/
    date: UTC_ISO_DateString
    /** Karma combinado (up - down) dos votos. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True se o usuário estava logado quando comentou, ou se verificou o comentário, ou se verificou sua sessão quando o comentário foi deixado. **/
    verified: boolean
    /** Data em que o comentário foi verificado. **/
    verifiedDate?: number
    /** Se um moderador marcou o comentário como revisado. **/
    reviewed: boolean
    /** A localização, ou codificação base64, do avatar. Só será base64 se esse valor foi enviado com SSO. **/
    avatarSrc?: string
    /** O comentário foi marcado manualmente ou automaticamente como spam? **/
    isSpam: boolean
    /** O comentário foi marcado automaticamente como spam? **/
    aiDeterminedSpam: boolean
    /** Há imagens no comentário? **/
    hasImages: boolean
    /** O número da página em que o comentário está para a direção de ordenação "Most Relevant". **/
    pageNumber: number
    /** O número da página em que o comentário está para a direção de ordenação "Oldest First". **/
    pageNumberOF: number
    /** O número da página em que o comentário está para a direção de ordenação "Newest First". **/
    pageNumberNF: number
    /** O comentário foi aprovado automaticamente ou manualmente? **/
    approved: boolean
    /** O código de locale (formato: en_us) do usuário quando o comentário foi escrito. **/
    locale: string
    /** As @mentions escritas no comentário que foram analisadas com sucesso. **/
    mentions?: CommentUserMention[]
    /** O domínio de onde o comentário se originou. **/
    domain?: string
    /** A lista opcional de moderation group ids associadas a este comentário. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Quando usuários são marcados em um comentário, a informação é armazenada em uma lista chamada `mentions`. Cada objeto nessa lista
tem a seguinte estrutura.

[inline-code-attrs-start title = 'O Objeto Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** O id do usuário. Para usuários SSO, este terá seu tenant id como prefixo. **/
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

#### Métodos HTTP Utilizados

**Create e Update ambos usam HTTP PUT e não POST!**

Como todas as nossas requisições contêm um ID, repetir a mesma requisição Create ou Update não deve criar novos objetos no seu lado.

Isso significa que essas chamadas são idempotentes e devem ser eventos PUT conforme a especificação HTTP.