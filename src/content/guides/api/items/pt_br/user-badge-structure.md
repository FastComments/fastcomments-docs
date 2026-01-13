`UserBadge` é um objeto que representa um distintivo atribuído a um usuário no sistema FastComments.

Distintivos podem ser atribuídos a usuários automaticamente com base em sua atividade (como contagem de comentários, tempo de resposta, status de veterano) ou manualmente por administradores do site.

A estrutura do objeto `UserBadge` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Identificador único para esta atribuição de distintivo do usuário */
    id: string
    /** ID do usuário a quem este distintivo está atribuído */
    userId: string
    /** ID da definição do distintivo no catálogo de distintivos do tenant */
    badgeId: string
    /** ID do tenant que criou/atribuiu este distintivo */
    fromTenantId: string
    /** Quando este distintivo foi criado (milissegundos desde a epoch) */
    createdAt?: number
    /** Quando este distintivo foi recebido pelo usuário (milissegundos desde a epoch) */
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
    /** O nome/etiqueta do distintivo */
    name?: string
    /** Descrição detalhada do distintivo */
    description?: string
    /** O texto exibido no distintivo */
    displayLabel?: string
    /** URL para uma imagem exibida no distintivo */
    displaySrc?: string
    /** Cor de fundo do distintivo (código hexadecimal) */
    backgroundColor?: string
    /** Cor da borda do distintivo (código hexadecimal) */
    borderColor?: string
    /** Cor do texto do distintivo (código hexadecimal) */
    textColor?: string
    /** Classe CSS adicional para estilização */
    cssClass?: string
    /** Para distintivos de veterano, o limite de tempo em milissegundos */
    veteranUserThresholdMillis?: number
    /** Indica se este distintivo é exibido nos comentários do usuário */
    displayedOnComments: boolean
    /** A ordem de exibição do distintivo */
    order?: number
}
[inline-code-end]
---