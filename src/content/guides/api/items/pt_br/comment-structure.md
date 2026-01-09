A `Comment` object representa um comentário deixado por um usuário.

A relação entre comentários pai e filho é definida via `parentId`.

A estrutura do objeto Comment é a seguinte:

[inline-code-attrs-start title = 'Estrutura do Comment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** SOMENTE LEITURA: Defina como true se o motor de spam determinou que o comentário era spam. **/
    aiDeterminedSpam?: boolean
    /** Se o comentário está aprovado para exibição. Defina como true ao salvar o comentário, caso contrário ficará oculto. **/
    approved?: boolean
    /** Avatar do usuário. **/
    avatarSrc?: string
    /** Comentários filhos. Nem sempre populado em todos os cenários. Usado quando asTree é definido como true via a API. **/
    children: Comment[]
    /** O comentário bruto do autor. **/
    comment: string
    /** SOMENTE LEITURA: O comentário do autor convertido para HTML. **/
    commentHTML?: string
    /** Email do autor. Obrigatório se comentários anônimos estiverem desativados. **/
    commenterEmail?: string
    /** Link do autor (por exemplo, seu blog). **/
    commenterLink?: string
    /** Nome do autor. Sempre exigido. Se não estiver disponível, defina algo como "Anônimo". **/
    commenterName: string
    /** A data em que o comentário foi feito, em epoch UTC. **/
    date: number
    /** O "rótulo de exibição" para o comentário - por exemplo "Admin", "Moderator", ou algo como "VIP User". **/
    displayLabel?: string
    /** O domínio onde o comentário foi postado. **/
    domain?: string
    /** SOMENTE LEITURA: O número de vezes que o comentário foi sinalizado. **/
    flagCount?: number
    /** As #hashtags escritas no comentário que foram parseadas com sucesso. Você também pode adicionar hashtags manualmente, para consulta, mas elas não serão exibidas automaticamente no texto do comentário. **/
    hashTags?: CommentHashTag[]
    /** SOMENTE LEITURA: O comentário contém imagens? **/
    hasImages?: boolean
    /** SOMENTE LEITURA: O comentário contém links? **/
    hasLinks?: boolean
    /** SOMENTE LEITURA: O id único do comentário. **/
    id: string
    /** Apenas na criação! Isso é hashado para armazenamento. **/
    ip?: string
    /** SOMENTE LEITURA: O usuário atual bloqueou o autor deste comentário? **/
    isBlocked?: boolean
    /** SOMENTE LEITURA: O comentário é de um admin? Definido automaticamente com base em userId. **/
    isByAdmin?: boolean
    /** SOMENTE LEITURA: O comentário é de um moderador? Definido automaticamente com base em userId. **/
    isByModerator?: boolean
    /** Defina como true se o comentário foi excluído de forma suave (um placeholder teve que ser mantido devido a alguma outra configuração). **/
    isDeleted?: boolean
    /** Defina como true se a conta do usuário foi excluída e o comentário precisou ser mantido. **/
    isDeletedUser?: boolean
    /** SOMENTE LEITURA: Foi sinalizado pelo usuário atualmente logado (contextUserId)? **/
    isFlagged?: boolean
    /** O comentário está fixado? **/
    isPinned?: boolean
    /** O comentário está bloqueado para novas respostas (moderadores ainda podem responder)? **/
    isLocked?: boolean
    /** O comentário é spam? **/
    isSpam?: boolean
    /** SOMENTE LEITURA: O comentário foi votado negativamente pelo usuário atual (contextUserId)? **/
    isVotedDown?: boolean
    /** SOMENTE LEITURA: O comentário foi votado positivamente pelo usuário atual (contextUserId)? **/
    isVotedUp?: boolean
    /** A localidade do comentário. Se não for fornecida, será derivada do cabeçalho HTTP Accept-Language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** SOMENTE LEITURA: As @menções escritas no comentário que foram parseadas com sucesso. **/
    mentions?: CommentUserMention[]
    /** Metadados opcionais associados ao comentário. **/
    meta?: Record<string, string | number | boolean>
    /** A lista opcional de ids de grupo de moderação associados a este comentário. **/
    moderationGroupIds?: string[]|null
    /** SOMENTE LEITURA: O id do objeto de voto que corresponde ao voto do usuário atual (contextUserId) neste comentário. **/
    myVoteId?: string
    /** Se notificações foram enviadas por este comentário para comentadores. Para evitar que notificações sejam enviadas em importações, defina isto como true. **/
    notificationSentForParent?: boolean
    /** Se notificações foram enviadas por este comentário para usuários do tenant. Para evitar envio de notificações em importações, defina isto como true. **/
    notificationSentForParentTenant?: boolean
    /** O título da página em que este comentário foi postado. **/
    pageTitle?: string
    /** Se estamos respondendo a um comentário, este é o ID ao qual estamos respondendo. **/
    parentId?: string|null
    /** Se o comentário está marcado como revisado. **/
    reviewed: boolean
    /** O id do tenant ao qual o comentário pertence. **/
    tenantId: string
    /** O usuário que escreveu o comentário. Criado automaticamente ao salvar um comentário com nome/email. **/
    userId?: string|null
    /** A URL do local em que este comentário é visível, como um post de blog. **/
    url: string
    /** Uma versão "limpa" do urlId que você nos passou. Ao salvar, você especifica este campo, mas ao buscar o comentário de volta ele será "limpo" e seu valor original movido para "urlIdRaw". **/
    urlId: string
    /** SOMENTE LEITURA: O urlId original que você nos passou. **/
    urlIdRaw?: string
    /** O usuário e este comentário são verificados? **/
    verified: boolean
    /** Número de votos positivos. **/
    votesUp?: number
    /** Número de votos negativos. **/
    votesDown?: number
    /** O "karma" do comentário (= votos positivos - votos negativos). **/
    votes?: number
}
[inline-code-end]

Alguns destes campos estão marcados `READONLY` - estes são retornados pela API mas não podem ser definidos.

### Estrutura do Texto do Comentário

Os comentários são escritos em uma variante de markdown do FastComments, que é apenas markdown mais tags no estilo `bbcode` para imagens, como `[img]path[/img]`.

O texto é armazenado em dois campos. O texto que o usuário digitou é armazenado sem modificações no campo `comment`. Este é renderizado e armazenado no campo `commentHTML`.

As tags HTML permitidas são `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

É recomendado renderizar o HTML, já que é um subconjunto muito pequeno de HTML; construir um renderizador é bastante direto. Existem várias bibliotecas para React Native e Flutter, por exemplo, para ajudar com isso

Você pode optar por renderizar o valor não normalizado do campo `comment`. [Um parser de exemplo está aqui.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

O parser de exemplo também pode ser ajustado para trabalhar com HTML e transformar as tags HTML em elementos esperados para renderizar na sua plataforma. 

### Marcação

Quando usuários são marcados em um comentário, a informação é armazenada em uma lista chamada `mentions`. Cada objeto nessa lista
tem a seguinte estrutura.

[inline-code-attrs-start title = 'O objeto de menções do Comment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** O id do usuário. Para usuários SSO, isso terá seu id do tenant prefixado. **/
    id: string
    /** O texto final da @menção, incluindo o símbolo @. **/
    tag: string
    /** O texto original da @menção, incluindo o símbolo @. **/
    rawTag: string
    /** Que tipo de usuário foi marcado. user = conta FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se o usuário optar por não receber notificações, isto ainda será definido como true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Quando hashtags são usadas e parseadas com sucesso, a informação é armazenada em uma lista chamada `hashTags`. Cada objeto nessa lista
tem a seguinte estrutura. Hashtags também podem ser adicionadas manualmente ao array `hashTags` do comentário para consulta, se `retain` estiver definido.

[inline-code-attrs-start title = 'O objeto de HashTag do Comment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** O id da hashtag. **/
    id: string
    /** O texto final da #hashtag, incluindo o símbolo #. **/
    tag: string
    /** Se a hashtag estiver associada a uma URL personalizada, isto será definido. **/
    url?: string
    /** Se deveríamos reter a hashtag, mesmo que não exista no texto do comentário, quando o comentário for atualizado. Útil para marcar comentários sem alterar o texto do comentário. **/
    retain?: boolean
}
[inline-code-end]

---