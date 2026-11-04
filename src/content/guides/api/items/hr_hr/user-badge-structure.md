`UserBadge` je objekt koji predstavlja značku dodijeljenu korisniku u FastComments sustavu.

Značke se korisnicima mogu dodjeljivati automatski na temelju njihove aktivnosti (kao što su broj komentara, vrijeme odgovora, status veterana) ili ručno od strane administratora stranice.

Struktura objekta `UserBadge` je sljedeća:

[inline-code-attrs-start title = 'Struktura UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Jedinstveni identifikator ove dodjele značke korisniku */
    id: string
    /** ID korisnika kojem je značka dodijeljena */
    userId: string
    /** ID definicije značke iz kataloga znački tenant-a */
    badgeId: string
    /** ID tenant-a koji je stvorio/dodijelio ovu značku */
    fromTenantId: string
    /** Kada je ova značka stvorena (milisekunde od epohe) */
    createdAt?: number
    /** Kada je korisnik primio ovu značku (milisekunde od epohe) */
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
    /** Za značke temeljene na pragovima, vrijednost praga */
    threshold?: number
    /** Naziv/oznaka značke */
    name?: string
    /** Detaljan opis značke */
    description?: string
    /** Tekst prikazan na znački */
    displayLabel?: string
    /** URL slike prikazane na znački */
    displaySrc?: string
    /** Boja pozadine značke (hex kod) */
    backgroundColor?: string
    /** Boja ruba značke (hex kod) */
    borderColor?: string
    /** Boja teksta značke (hex kod) */
    textColor?: string
    /** Dodatna CSS klasa za stiliziranje */
    cssClass?: string
    /** Za značke veterana, prag vremena u milisekundama */
    veteranUserThresholdMillis?: number
    /** Je li ova značka prikazana na komentarima korisnika */
    displayedOnComments: boolean
    /** Redoslijed prikaza značke */
    order?: number
    /** Ako je postavljeno, ova se značka prikazuje samo na stranici s odgovarajućim urlId. Null za globalne značke. */
    urlId?: string | null
}
[inline-code-end]