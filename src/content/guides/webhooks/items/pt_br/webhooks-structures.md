A única estrutura enviada via webhooks é o objeto WebhookComment, descrito em TypeScript abaixo.

#### Estrutura do Objeto WebhookComment

##### Estrutura do Evento "Create"
O corpo da requisição do evento "create" é um objeto WebhookComment.

##### Estrutura do Evento "Update"
O corpo da requisição do evento "update" é um objeto WebhookComment.

##### Estrutura do Evento "Delete"
O corpo da requisição do evento "delete" é um objeto WebhookComment.

    Alteração a partir de 14 de nov de 2023
    Anteriormente, o corpo da requisição do evento "delete" continha apenas o id do comentário. Agora contém o comentário completo no momento da exclusão.

Cada chave está sempre presente no corpo. Quando o comentário não tem valor para um campo, o corpo contém `null` (ou `false` para booleanos e `[]` para listas), de modo que a estrutura de entrega nunca varia de um comentário para outro.

[inline-code-attrs-start title = 'O Objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** O id do comentário. **/
    id: string
    /** O id ou URL que identifica o thread do comentário. Normalizado. **/
    urlId: string
    /** A URL que aponta para onde o comentário foi deixado. **/
    url: string | null
    /** O id do usuário que deixou o comentário. Se SSO, prefixado com o id do tenant. **/
    userId: string | null
    /** O email do usuário que deixou o comentário. **/
    commenterEmail: string | null
    /** O nome do usuário que aparece no widget de comentário. Com SSO, pode ser displayName. **/
    commenterName: string
    /** Texto bruto do comentário. **/
    comment: string
    /** Texto do comentário após o parsing. **/
    commentHTML: string
    /** Id externo do comentário. **/
    externalId: string | null
    /** O id do comentário pai. **/
    parentId: string | null
    /** A data UTC quando o comentário foi deixado. **/
    date: UTC_ISO_DateString
    /** Karma combinado (up - down) dos votos. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Verdadeiro se o usuário estava logado ao comentar, ou verificou o comentário, ou se verificou a sessão quando o comentário foi deixado. **/
    verified: boolean
    /** A data UTC quando o comentário foi verificado. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Se um moderador marcou o comentário como revisado. **/
    reviewed: boolean
    /** A localização, ou codificação base64, do avatar. Será base64 somente se esse foi o valor passado com SSO. **/
    avatarSrc: string | null
    /** O comentário foi marcado manual ou automaticamente como spam? **/
    isSpam: boolean
    /** O comentário foi marcado automaticamente como spam? **/
    aiDeterminedSpam: boolean
    /** Existem imagens no comentário? **/
    hasImages: boolean
    /** O número da página onde o comentário está para a ordenação "Mais Relevante". **/
    pageNumber: number | null
    /** O número da página onde o comentário está para a ordenação "Mais Antigos Primeiro". **/
    pageNumberOF: number | null
    /** O número da página onde o comentário está para a ordenação "Mais Recentes Primeiro". **/
    pageNumberNF: number | null
    /** O comentário foi aprovado automática ou manualmente? **/
    approved: boolean
    /** O código de localidade (formato: en_us) do usuário quando o comentário foi escrito. **/
    locale: string | null
    /** As @menções escritas no comentário que foram analisadas com sucesso. Vazio quando não houver nenhuma. **/
    mentions: CommentUserMention[]
    /** O domínio de onde o comentário vem. **/
    domain: string | null
    /** Os ids dos grupos de moderação associados a este comentário. Vazio quando não houver nenhum. **/
    moderationGroupIds: string[]
}
[inline-code-end]

Quando usuários são marcados em um comentário, a informação é armazenada em uma lista chamada `mentions`. Cada objeto nessa lista tem a seguinte estrutura.

[inline-code-attrs-start title = 'O Objeto Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** O id do usuário. Para usuários SSO, este terá o id do tenant prefixado. **/
    id: string
    /** O texto final da tag @mention, incluindo o símbolo @. **/
    tag: string
    /** O texto original da tag @mention, incluindo o símbolo @. **/
    rawTag: string
    /** Que tipo de usuário foi marcado. user = conta FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se o usuário optou por não receber notificações, isso ainda será definido como true. **/
    sent: boolean
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
| `token` | Seu segredo da API |
| `X-FastComments-Timestamp` | Timestamp Unix (segundos) quando a requisição foi assinada |
| `X-FastComments-Signature` | Assinatura HMAC-SHA256 (`sha256=<hex>`) |

Veja [Segurança e Tokens de API](/guide-webhooks.html#webhooks-api-tokens) para informações sobre como verificar a assinatura HMAC.