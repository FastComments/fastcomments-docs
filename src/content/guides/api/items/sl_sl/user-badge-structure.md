`UserBadge` je objekt, ki predstavlja značko, dodeljeno uporabniku v sistemu FastComments.

Značke se lahko uporabnikom dodelijo samodejno na podlagi njihove aktivnosti (kot so število komentarjev, čas odgovora, status veterana) ali ročno s strani skrbnikov spletnega mesta.

Struktura objekta `UserBadge` je naslednja:

[inline-code-attrs-start title = 'Struktura objekta UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Enoličen identifikator te dodelitve uporabniške značke */
    id: string
    /** ID uporabnika, kateremu je ta značka dodeljena */
    userId: string
    /** ID definicije značke iz kataloga značk najemnika */
    badgeId: string
    /** ID najemnika, ki je ustvaril/dodelil to značko */
    fromTenantId: string
    /** Kdaj je bila ta značka ustvarjena (milisekunde od epohe) */
    createdAt?: number
    /** Kdaj je ta značka prejeta s strani uporabnika (milisekunde od epohe) */
    receivedAt?: number
    /** 
     * Tip značke: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Za značke, ki temeljijo na pragih, vrednost praga */
    threshold?: number
    /** Ime/oznaka značke */
    name?: string
    /** Podroben opis značke */
    description?: string
    /** Besedilo, prikazano na znački */
    displayLabel?: string
    /** URL do slike, prikazane na znački */
    displaySrc?: string
    /** Barva ozadja značke (heksadecimalna koda) */
    backgroundColor?: string
    /** Barva obrobe značke (heksadecimalna koda) */
    borderColor?: string
    /** Barva besedila značke (heksadecimalna koda) */
    textColor?: string
    /** Dodatna CSS razred za stiliranje */
    cssClass?: string
    /** Za veteranske značke, časovni prag v milisekundah */
    veteranUserThresholdMillis?: number
    /** Ali se ta značka prikazuje ob uporabnikovih komentarjih */
    displayedOnComments: boolean
    /** Vrstni red prikaza značke */
    order?: number
}
[inline-code-end]