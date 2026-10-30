A única estrutura enviada via webhooks é o objeto WebhookComment, descrita em TypeScript abaixo.

#### Estrutura do Objeto WebhookComment

##### Estrutura do Evento "Create"
O corpo da requisição do evento "create" é um objeto WebhookComment.

##### Estrutura do Evento "Update"
O corpo da requisição do evento "update" é um objeto WebhookComment.

##### Estrutura do Evento "Delete"
O corpo da requisição do evento "delete" é um objeto WebhookComment.

Alteração a partir de 14 de nov de 2023  
Anteriormente, o corpo da requisição do evento "delete" continha apenas o id do comentário. Agora contém o comentário completo no momento da exclusão.

Cada chave está sempre presente no corpo. Quando o comentário não tem valor para um campo, o corpo contém `null`  
(ou `false` para booleanos e `[]` para listas), de modo que a estrutura de uma entrega nunca varia de um comentário para outro.

[inline-code-attrs-start title = 'Objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
    /** The badges shown next to the commenter's name on this comment, as of when the event was created. Empty when there are none. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Quando usuários são marcados em um comentário, a informação é armazenada em uma lista chamada `mentions`. Cada objeto nessa lista
tem a seguinte estrutura.

[inline-code-attrs-start title = 'Objeto de Menções do Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

Cada entrada na lista `badges` tem a seguinte estrutura. Veja o [Guia de Badges](/guide-badges.html) para os tipos de badge.

[inline-code-attrs-start title = 'Objeto de Badge do Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** The id of the badge awarded to this user. **/
    id: string
    /** The badge type. **/
    type: number
    /** The badge description, shown when hovering over the badge. **/
    description: string
    /** The text shown on the badge. **/
    displayLabel?: string | null
    /** The image shown for the badge, if it uses one. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** An extra CSS class applied to the badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### Métodos HTTP

Você pode configurar o método HTTP para cada tipo de evento webhook no painel de administração:

- **Evento de Criação**: POST ou PUT (padrão: PUT)
- **Evento de Atualização**: POST ou PUT (padrão: PUT)
- **Evento de Exclusão**: DELETE, POST ou PUT (padrão: DELETE)

Como todas as requisições contêm um ID, as operações de Criação e Atualização são idempotentes por padrão (PUT). Repetir a mesma requisição de Criação ou Atualização não deve criar objetos duplicados no seu lado.

#### Cabeçalhos da Requisição

Cada requisição webhook inclui os seguintes cabeçalhos:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Seu Segredo de API |
| `X-FastComments-Timestamp` | Timestamp Unix (segundos) quando a requisição foi assinada |
| `X-FastComments-Signature` | assinatura HMAC-SHA256 (`sha256=<hex>`) |

Veja [Segurança & Tokens de API](/guide-webhooks.html#webhooks-api-tokens) para informações sobre como verificar a assinatura HMAC.