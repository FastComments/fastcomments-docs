`UserBadge` er et objekt, der repræsenterer et badge tildelt en bruger i FastComments-systemet.

Badges kan tildeles brugere automatisk baseret på deres aktivitet (såsom antal kommentarer, svartid, veteranstatur) eller manuelt af webstedsadministratorer.

Strukturen for `UserBadge`-objektet er som følger:

[inline-code-attrs-start title = 'UserBadge-struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Unik identifikator for denne bruger-badge-tildeling */
    id: string
    /** ID på den bruger, som dette badge er tildelt */
    userId: string
    /** ID på badgedefinitionen fra tenantens badge-katalog */
    badgeId: string
    /** ID på den tenant, der oprettede/tildelte dette badge */
    fromTenantId: string
    /** Hvornår dette badge blev oprettet (millisekunder siden epoch) */
    createdAt?: number
    /** Hvornår dette badge blev modtaget af brugeren (millisekunder siden epoch) */
    receivedAt?: number
    /** 
     * Badge-typen: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** For tærskelbaserede badges, tærskelværdien */
    threshold?: number
    /** Navn/etiket på badget */
    name?: string
    /** Detaljeret beskrivelse af badget */
    description?: string
    /** Teksten vist på badget */
    displayLabel?: string
    /** URL til et billede vist på badget */
    displaySrc?: string
    /** Baggrundsfarve for badget (hex-kode) */
    backgroundColor?: string
    /** Kantfarve for badget (hex-kode) */
    borderColor?: string
    /** Tekstfarve for badget (hex-kode) */
    textColor?: string
    /** Yderligere CSS-klasse til styling */
    cssClass?: string
    /** For veteran-badges, tidsgrænsen i millisekunder */
    veteranUserThresholdMillis?: number
    /** Om dette badge vises på brugerens kommentarer */
    displayedOnComments: boolean
    /** Visningsrækkefølgen for badget */
    order?: number
    /** Hvis sat, vises dette badge kun på siden med matchende urlId. Null for globale badges. */
    urlId?: string | null
}
[inline-code-end]