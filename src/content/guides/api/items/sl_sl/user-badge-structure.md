`UserBadge` je objekt, ki predstavlja značko dodeljeno uporabniku v sistemu FastComments.

Značke se lahko dodelijo uporabnikom samodejno, glede na njihovo dejavnost (na primer število komentarjev, čas odgovora, status veterana) ali ročno s strani skrbnikov spletnega mesta.

Struktura objekta `UserBadge` je naslednja:

[inline-code-attrs-start title = 'Struktura objekta UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Enolični identifikator te dodelitve značke uporabnika */
    id: string
    /** ID uporabnika, kateremu je ta značka dodeljena */
    userId: string
    /** ID definicije značke iz kataloga značk najemnika */
    badgeId: string
    /** ID najemnika, ki je ustvaril/dodelil to značko */
    fromTenantId: string
    /** Kdaj je bila ta značka ustvarjena (milisekunde od Unixove epohe) */
    createdAt?: number
    /** Kdaj je uporabnik prejel to značko (milisekunde od Unixove epohe) */
    receivedAt?: number
    /** 
     * Vrsta značke: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Za značke, ki temeljijo na pragu, vrednost praga */
    threshold?: number
    /** Ime/označba značke */
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
    /** Dodatni CSS razred za stiliziranje */
    cssClass?: string
    /** Za veteranske značke, časovni prag v milisekundah */
    veteranUserThresholdMillis?: number
    /** Ali je ta značka prikazana na uporabnikovih komentarjih */
    displayedOnComments: boolean
    /** Vrstni red prikaza značke */
    order?: number
    /** Če je nastavljeno, je ta značka prikazana samo na strani z ustreznim urlId. Null za globalne značke. */
    urlId?: string | null
}
[inline-code-end]