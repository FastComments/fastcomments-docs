`UserBadge` je objekt koji predstavlja značku dodijeljenu korisniku u FastComments sustavu.

Značke se mogu automatski dodijeliti korisnicima na temelju njihove aktivnosti (npr. broja komentara, vremena odgovora, statusa veterana) ili ručno od strane administratora stranice.

The structure for the `UserBadge` object is as follows:

[inline-code-attrs-start title = 'Struktura UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Jedinstveni identifikator za ovu dodjelu značke korisniku */
    id: string
    /** ID korisnika kojem je ova značka dodijeljena */
    userId: string
    /** ID definicije značke iz kataloga znački tenanta */
    badgeId: string
    /** ID tenanta koji je kreirao/dodijelio ovu značku */
    fromTenantId: string
    /** Kada je ova značka kreirana (milisekunde od epohe) */
    createdAt?: number
    /** Kada je ova značka primljena od strane korisnika (milisekunde od epohe) */
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
    /** Za značke temeljene na pragu, vrijednost praga */
    threshold?: number
    /** Naziv/etiketa značke */
    name?: string
    /** Detaljan opis značke */
    description?: string
    /** Tekst prikazan na znački */
    displayLabel?: string
    /** URL slike koja se prikazuje na znački */
    displaySrc?: string
    /** Boja pozadine za značku (heksadecimalni kod) */
    backgroundColor?: string
    /** Boja obruba za značku (heksadecimalni kod) */
    borderColor?: string
    /** Boja teksta za značku (heksadecimalni kod) */
    textColor?: string
    /** Dodatna CSS klasa za stiliziranje */
    cssClass?: string
    /** Za veteranske značke, vremenski prag u milisekundama */
    veteranUserThresholdMillis?: number
    /** Prikazuje li se ova značka na komentarima korisnika */
    displayedOnComments: boolean
    /** Redoslijed prikaza značke */
    order?: number
    /** If set, this badge is only displayed on the page with the matching urlId. Null for global badges. */
    urlId?: string | null
}
[inline-code-end]