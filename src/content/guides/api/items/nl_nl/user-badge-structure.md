---
`UserBadge` is een object dat een badge voorstelt die aan een gebruiker is toegewezen in het FastComments-systeem.

Badges kunnen automatisch aan gebruikers worden toegekend op basis van hun activiteit (zoals aantal reacties, reactietijd, Veteran-status) of handmatig door sitebeheerders.

De structuur van het `UserBadge`-object is als volgt:

[inline-code-attrs-start title = 'UserBadge-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Unieke identificatie voor deze gebruikersbadge-toewijzing */
    id: string
    /** ID van de gebruiker aan wie deze badge is toegewezen */
    userId: string
    /** ID van de badge-definitie uit de badgecatalogus van de tenant */
    badgeId: string
    /** ID van de tenant die deze badge heeft gemaakt/toegewezen */
    fromTenantId: string
    /** Wanneer deze badge is gemaakt (milliseconden sinds epoch) */
    createdAt?: number
    /** Wanneer deze badge door de gebruiker is ontvangen (milliseconden sinds epoch) */
    receivedAt?: number
    /** 
     * Het badgetype: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Voor op drempel gebaseerde badges, de drempelwaarde */
    threshold?: number
    /** De naam/label van de badge */
    name?: string
    /** Gedetailleerde beschrijving van de badge */
    description?: string
    /** De tekst die op de badge wordt weergegeven */
    displayLabel?: string
    /** URL naar een afbeelding die op de badge wordt getoond */
    displaySrc?: string
    /** Achtergrondkleur voor de badge (hex-code) */
    backgroundColor?: string
    /** Randkleur voor de badge (hex-code) */
    borderColor?: string
    /** Tekstkleur voor de badge (hex-code) */
    textColor?: string
    /** Extra CSS-klasse voor styling */
    cssClass?: string
    /** Voor Veteran-badges, de tijdsdrempel in milliseconden */
    veteranUserThresholdMillis?: number
    /** Of deze badge wordt weergegeven bij de reacties van de gebruiker */
    displayedOnComments: boolean
    /** De weergavevolgorde van de badge */
    order?: number
    /** Als ingesteld, wordt deze badge alleen weergegeven op de pagina met de overeenkomende urlId. Null voor globale badges. */
    urlId?: string | null
}
[inline-code-end]
---