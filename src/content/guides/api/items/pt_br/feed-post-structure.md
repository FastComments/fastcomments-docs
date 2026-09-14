Um objeto `FeedPost` representa uma postagem em um feed do FastComments. Um feed é um fluxo de postagens com seus próprios tópicos de comentários, renderizado pelo widget Feed. Cada postagem tem um autor, conteúdo rico opcional, mídia e links, e pode ser marcada para que um feed possa ser filtrado.

A estrutura do objeto `FeedPost` é a seguinte:

[inline-code-attrs-start title = 'Estrutura FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** APENAS LEITURA **/
    _id: string
    /** APENAS LEITURA **/
    tenantId: string
    title?: string
    /** O id do usuário FastComments ou SSO que criou a postagem. **/
    fromUserId?: string
    /** Preenchido a partir do usuário quando não definido. **/
    fromUserDisplayName?: string | null
    /** APENAS LEITURA. Preenchido a partir do usuário. **/
    fromUserAvatar?: string | null
    /** Usado para filtrar um feed. **/
    tags?: string[]
    /** Peso de ordenação dentro de um feed. Valores maiores são exibidos primeiro. **/
    weight?: number
    /** Pares chave/valor de formato livre para seu próprio uso. **/
    meta?: Record<string, string>
    /** HTML sanitizado. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** APENAS LEITURA **/
    createdAt: string
    /** APENAS LEITURA. Tipo de reação a contar. **/
    reacts?: Record<string, number>
    /** APENAS LEITURA **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Para onde o item de mídia aponta quando clicado. **/
    linkUrl?: string
    /** Uma entrada por variação. O widget escolhe a melhor opção. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** O texto do link, como "Inscreva‑se agora". **/
    text?: string
    /** Um título exibido com o link. **/
    title?: string
    /** Uma descrição exibida com o link. **/
    description?: string
    url?: string
}
[inline-code-end]

Notas:

- Alguns desses campos são marcados como `READONLY` – eles são retornados pela API, mas não podem ser definidos.
- Os comentários em uma postagem são comentários regulares cujo `urlId` é `post:` seguido pelo `_id` da postagem. Use esse valor com a API de Comentários para ler ou criar comentários em uma postagem.