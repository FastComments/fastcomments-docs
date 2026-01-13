`UserBadge` je objekt koji predstavlja značku dodijeljenu korisniku u sustavu FastComments.

Značke se mogu dodijeliti korisnicima automatski na temelju njihove aktivnosti (kao što su broj komentara, vrijeme odgovora, status veterana) ili ručno od strane administratora stranice.

Struktura objekta `UserBadge` je sljedeća:

[inline-code-attrs-start title = 'Struktura UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Jedinstveni identifikator za ovu dodjelu značke korisniku */
    id: string
    /** ID korisnika kojem je ova značka dodijeljena */
    userId: string
    /** ID definicije značke iz kataloga znački tenant-a */
    badgeId: string
    /** ID tenanta koji je stvorio/dodijelio ovu značku */
    fromTenantId: string
    /** Kada je ova značka stvorena (milisekunde od epohe) */
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
    /** Za značke bazirane na pragovima, vrijednost praga */
    threshold?: number
    /** Naziv/oznaka značke */
    name?: string
    /** Detaljan opis značke */
    description?: string
    /** Tekst prikazan na značci */
    displayLabel?: string
    /** URL slike prikazane na značci */
    displaySrc?: string
    /** Boja pozadine značke (hex kod) */
    backgroundColor?: string
    /** Boja ruba značke (hex kod) */
    borderColor?: string
    /** Boja teksta značke (hex kod) */
    textColor?: string
    /** Dodatna CSS klasa za stiliziranje */
    cssClass?: string
    /** Za veteranske značke, vremenski prag u milisekundama */
    veteranUserThresholdMillis?: number
    /** Prikazuje li se ova značka na korisnikovim komentarima */
    displayedOnComments: boolean
    /** Redoslijed prikaza značke */
    order?: number
}
[inline-code-end]
---