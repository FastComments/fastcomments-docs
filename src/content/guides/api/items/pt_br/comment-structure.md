Um objeto `Comment` representa um comentário deixado por um usuário.

A relação entre comentários pai e filhos é definida via `parentId`.

A estrutura do objeto Comment é a seguinte:

[inline-code-attrs-start title = 'Estrutura do Comment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Definido como true se o motor de spam determinou que o comentário era spam. **/
    aiDeterminedSpam?: boolean
    /** Se o comentário está aprovado para exibição. Defina como true ao salvar o comentário, caso contrário ele ficará oculto. **/
    approved?: boolean
    /** O avatar do usuário. **/
    avatarSrc?: string
    /** Comentários filhos. Não preenchido em todos os cenários. Usado quando asTree é definido como true via API. **/
    children: Comment[]
    /** O comentário bruto do comentarista. **/
    comment: string
    /** READONLY: O comentário do comentarista convertido em HTML. **/
    commentHTML?: string
    /** O email do comentarista. Obrigatório se comentários anônimos estiverem desativados. **/
    commenterEmail?: string
    /** O link do comentarista (por exemplo, o blog dele). **/
    commenterLink?: string
    /** O nome do comentarista. Sempre obrigatório. Se não estiver disponível, defina como algo como "Anonymous". **/
    commenterName: string
    /** A data em que o comentário foi deixado, em UTC epoch. **/
    date: number
    /** O "rótulo de exibição" do comentário - por exemplo "Admin", "Moderator", ou algo como "VIP User". **/
    displayLabel?: string
    /** O domínio onde o comentário foi postado. **/
    domain?: string
    /** READONLY: O número de vezes que o comentário foi marcado. **/
    flagCount?: number
    /** As #hashtags escritas no comentário que foram analisadas com sucesso. Você também pode adicionar hashtags manualmente, para consulta, mas elas não serão exibidas automaticamente no texto do comentário. **/
    hashTags?: CommentHashTag[]
    /** READONLY: O comentário contém imagens? **/
    hasImages?: boolean
    /** READONLY: O comentário contém links? **/
    hasLinks?: boolean
    /** READONLY: O id único do comentário. **/
    id: string
    /** Somente na criação! Este valor é hasheado para armazenamento. **/
    ip?: string
    /** READONLY: O usuário atual bloqueou o usuário que escreveu este comentário? **/
    isBlocked?: boolean
    /** READONLY: O comentário é de um admin? Definido automaticamente com base no userId. **/
    isByAdmin?: boolean
    /** READONLY: O comentário é de um moderador? Definido automaticamente com base no userId. **/
    isByModerator?: boolean
    /** Definido como true se o comentário foi excluído de forma suave (um placeholder precisou ser deixado devido a alguma outra configuração). **/
    isDeleted?: boolean
    /** Definido como true se a conta do usuário foi excluída e o comentário precisou ser mantido. **/
    isDeletedUser?: boolean
    /** READONLY: Foi marcado pelo usuário atualmente logado (contextUserId)? **/
    isFlagged?: boolean
    /** O comentário está fixado? **/
    isPinned?: boolean
    /** O comentário está bloqueado? Quando true, ninguém (incluindo moderadores) pode responder, editar ou excluí-lo até que seja desbloqueado. **/
    isLocked?: boolean
    /** O comentário é spam? **/
    isSpam?: boolean
    /** READONLY: O comentário recebeu voto negativo para o usuário atual (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: O comentário recebeu voto positivo para o usuário atual (contextUserId)? **/
    isVotedUp?: boolean
    /** O locale do comentário. Se não for fornecido, será derivado do header HTTP accept-language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: As @menções escritas no comentário que foram analisadas com sucesso. **/
    mentions?: CommentUserMention[]
    /** Metadados opcionais associados ao comentário. **/
    meta?: Record<string, string | number | boolean>
    /** A lista opcional de ids de grupos de moderação associados a este comentário. **/
    moderationGroupIds?: string[]|null
    /** READONLY: O id do objeto de voto que corresponde ao voto do usuário atual (contextUserId) neste comentário. **/
    myVoteId?: string
    /** Se notificações foram enviadas para este comentário para os comentaristas. Para evitar que notificações sejam enviadas em importações, defina isto como true. **/
    notificationSentForParent?: boolean
    /** Se notificações foram enviadas para este comentário para os usuários do tenant. Para evitar que notificações sejam enviadas em importações, defina isto como true. **/
    notificationSentForParentTenant?: boolean
    /** O título da página onde este comentário estava. **/
    pageTitle?: string
    /** Se estamos respondendo a um comentário, este é o ID ao qual estamos respondendo. **/
    parentId?: string|null
    /** Se o comentário está marcado como revisado. **/
    reviewed: boolean
    /** O id do tenant ao qual o comentário pertence. **/
    tenantId: string
    /** O usuário que escreveu o comentário. Criado automaticamente ao salvar um comentário com nome/email. **/
    userId?: string|null
    /** A URL do local onde este comentário é visível, como um post de blog. **/
    url: string
    /** Uma versão "limpa" do urlId que você nos passou. Ao salvar, você especifica este campo, mas ao buscar o comentário de volta ele estará "limpo" e seu valor original movido para "urlIdRaw". **/
    urlId: string
    /** READONLY: O urlId original que você nos passou. **/
    urlIdRaw?: string
    /** O usuário e este comentário estão verificados? **/
    verified: boolean
    /** Número de votos positivos. **/
    votesUp?: number
    /** Número de votos negativos. **/
    votesDown?: number
    /** O "karma" do comentário (= votos positivos - votos negativos). **/
    votes?: number
}
[inline-code-end]

Alguns desses campos estão marcados como `READONLY` - eles são retornados pela API, mas não podem ser definidos.

### Estrutura do Texto do Comentário

Os comentários são escritos em uma versão do markdown da FastComments, que é apenas markdown mais tags tradicionais no estilo `bbcode` para imagens, como `[img]path[/img]`.

O texto é armazenado em dois campos. O texto que o usuário digitou é armazenado sem modificações no campo `comment`. Este é renderizado e armazenado no campo `commentHTML`.

As tags HTML permitidas são `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

É recomendado renderizar o HTML, já que é um subconjunto muito pequeno de HTML, e construir um renderizador é bem simples. Existem várias bibliotecas para React Native e Flutter, por exemplo, para ajudar com isso.

Você pode optar por renderizar o valor não normalizado do campo `comment`. [Um exemplo de parser está aqui.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

O parser de exemplo também pode ser ajustado para funcionar com HTML, transformando as tags HTML nos elementos esperados para renderizar na sua plataforma.

### Marcações (Tagging)

Quando os usuários são marcados em um comentário, a informação é armazenada em uma lista chamada `mentions`. Cada objeto dessa lista
tem a seguinte estrutura.

[inline-code-attrs-start title = 'O Objeto de Menções do Comentário'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** O id do usuário. Para usuários SSO, este terá o id do seu tenant como prefixo. **/
    id: string
    /** O texto final da tag de @menção, incluindo o símbolo @. **/
    tag: string
    /** O texto original da tag de @menção, incluindo o símbolo @. **/
    rawTag: string
    /** Que tipo de usuário foi marcado. user = conta FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se o usuário desativar as notificações, isto ainda será definido como true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Quando hashtags são usadas e analisadas com sucesso, a informação é armazenada em uma lista chamada `hashTags`. Cada objeto dessa lista
tem a seguinte estrutura. Hashtags também podem ser adicionadas manualmente ao array `hashTags` do comentário para consulta, se `retain` estiver definido.

[inline-code-attrs-start title = 'O Objeto HashTag do Comentário'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** O id da hashtag. **/
    id: string
    /** O texto final da tag de #hashtag, incluindo o símbolo #. **/
    tag: string
    /** Se a hashtag estiver associada a uma URL personalizada, isto será definido. **/
    url?: string
    /** Se devemos manter a hashtag, mesmo que ela não exista no texto do comentário, quando o comentário for atualizado. Útil para marcar comentários sem alterar o texto do comentário. **/
    retain?: boolean
}
[inline-code-end]