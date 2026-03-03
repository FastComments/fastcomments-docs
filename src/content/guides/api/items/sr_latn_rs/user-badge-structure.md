`UserBadge` je objekat koji predstavlja značku dodeljenu korisniku u FastComments sistemu.

Značke mogu biti dodeljene korisnicima automatski na osnovu njihove aktivnosti (kao što su broj komentara, vreme odgovora, veteranski status) ili ručno od strane administratora sajta.

Struktura za `UserBadge` objekat je sledeća:

[inline-code-attrs-start title = 'Struktura UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Jedinstveni identifikator za ovu dodelu značke korisniku */
    id: string
    /** ID korisnika kojem je ova značka dodeljena */
    userId: string
    /** ID definicije značke iz kataloga znački tenanta */
    badgeId: string
    /** ID tenanta koji je kreirao/dodelio ovu značku */
    fromTenantId: string
    /** Kada je ova značka kreirana (milisekunde od epohe) */
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
    /** Za značke zasnovane na pragu, vrednost praga */
    threshold?: number
    /** Ime/oznaka značke */
    name?: string
    /** Detaljan opis značke */
    description?: string
    /** Tekst koji se prikazuje na znački */
    displayLabel?: string
    /** URL slike koja se prikazuje na znački */
    displaySrc?: string
    /** Boja pozadine za značku (hex kod) */
    backgroundColor?: string
    /** Boja ivice značke (hex kod) */
    borderColor?: string
    /** Boja teksta na znački (hex kod) */
    textColor?: string
    /** Dodatna CSS klasa za stilizovanje */
    cssClass?: string
    /** Za veteransku značku, vremenski prag u milisekundama */
    veteranUserThresholdMillis?: number
    /** Da li se ova značka prikazuje na komentarima korisnika */
    displayedOnComments: boolean
    /** Redosled prikaza značke */
    order?: number
    /** Ako je postavljeno, ova značka se prikazuje samo na stranici sa odgovarajućim urlId. Null za globalne značke. */
    urlId?: string | null
}
[inline-code-end]
---