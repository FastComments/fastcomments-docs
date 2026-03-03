`UserBadge` est un objet qui représente un badge attribué à un utilisateur dans le système FastComments.

Les badges peuvent être attribués aux utilisateurs automatiquement en fonction de leur activité (par exemple le nombre de commentaires, le temps de réponse, le statut de vétéran) ou manuellement par les administrateurs du site.

La structure de l'objet `UserBadge` est la suivante :

[inline-code-attrs-start title = 'Structure de l'objet UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Identifiant unique pour cette attribution de badge utilisateur */
    id: string
    /** ID de l'utilisateur auquel ce badge est attribué */
    userId: string
    /** ID de la définition du badge dans le catalogue de badges du locataire */
    badgeId: string
    /** ID du locataire qui a créé/attribué ce badge */
    fromTenantId: string
    /** Quand ce badge a été créé (millisecondes depuis l'époque Unix) */
    createdAt?: number
    /** Quand ce badge a été reçu par l'utilisateur (millisecondes depuis l'époque Unix) */
    receivedAt?: number
    /** 
     * Le type de badge : 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Pour les badges basés sur un seuil, la valeur du seuil */
    threshold?: number
    /** Le nom/libellé du badge */
    name?: string
    /** Description détaillée du badge */
    description?: string
    /** Le texte affiché sur le badge */
    displayLabel?: string
    /** URL vers une image affichée sur le badge */
    displaySrc?: string
    /** Couleur d'arrière-plan du badge (code hexadécimal) */
    backgroundColor?: string
    /** Couleur de la bordure du badge (code hexadécimal) */
    borderColor?: string
    /** Couleur du texte du badge (code hexadécimal) */
    textColor?: string
    /** Classe CSS supplémentaire pour le style */
    cssClass?: string
    /** Pour les badges vétéran, le seuil de temps en millisecondes */
    veteranUserThresholdMillis?: number
    /** Indique si ce badge est affiché sur les commentaires de l'utilisateur */
    displayedOnComments: boolean
    /** L'ordre d'affichage du badge */
    order?: number
    /** Si défini, ce badge n'est affiché que sur la page ayant le même urlId. Null pour les badges globaux. */
    urlId?: string | null
}
[inline-code-end]
---