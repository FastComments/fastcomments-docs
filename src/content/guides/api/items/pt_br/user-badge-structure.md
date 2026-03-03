`UserBadge` é um objeto que representa um distintivo atribuído a um usuário no sistema FastComments.

Os distintivos podem ser atribuídos a usuários automaticamente com base em sua atividade (como contagem de comentários, tempo de resposta, status de veterano) ou manualmente por administradores do site.

A estrutura do objeto `UserBadge` é a seguinte:

[inline-code-attrs-start title = 'Estrutura de UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Identificador único para esta atribuição de distintivo do usuário */
    id: string
    /** ID do usuário a quem este distintivo foi atribuído */
    userId: string
    /** ID da definição do distintivo do catálogo de distintivos do tenant */
    badgeId: string
    /** ID do tenant que criou/atribuído este distintivo */
    fromTenantId: string
    /** Quando este distintivo foi criado (milissegundos desde epoch) */
    createdAt?: number
    /** Quando este distintivo foi recebido pelo usuário (milissegundos desde epoch) */
    receivedAt?: number
    /** 
     * O tipo do distintivo: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Para distintivos baseados em limiar, o valor do limiar */
    threshold?: number
    /** O nome/rótulo do distintivo */
    name?: string
    /** Descrição detalhada do distintivo */
    description?: string
    /** O texto exibido no distintivo */
    displayLabel?: string
    /** URL para uma imagem exibida no distintivo */
    displaySrc?: string
    /** Cor de fundo do distintivo (código hex) */
    backgroundColor?: string
    /** Cor da borda do distintivo (código hex) */
    borderColor?: string
    /** Cor do texto do distintivo (código hex) */
    textColor?: string
    /** Classe CSS adicional para estilização */
    cssClass?: string
    /** Para distintivos de veterano, o limiar de tempo em milissegundos */
    veteranUserThresholdMillis?: number
    /** Se este distintivo é exibido nos comentários do usuário */
    displayedOnComments: boolean
    /** A ordem de exibição do distintivo */
    order?: number
    /** Se definido, este distintivo é exibido apenas na página com o urlId correspondente. Null para distintivos globais. */
    urlId?: string | null
}
[inline-code-end]