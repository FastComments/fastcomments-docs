`UserBadge` je objekat koji predstavlja značku dodeljenu korisniku u FastComments sistemu.

Značke mogu biti dodeljene korisnicima automatski na osnovu njihove aktivnosti (kao što su broj komentara, vreme odgovora, status veterana) ili ručno od strane administratora sajta.

Struktura objekta `UserBadge` je sledeća:

[inline-code-attrs-start title = 'Struktura objekta UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Jedinstveni identifikator za ovu dodelu značke korisniku */
    id: string
    /** ID korisnika kome je ova značka dodeljena */
    userId: string
    /** ID definicije značke iz tenant-ovog kataloga */
    badgeId: string
    /** ID tenant-a koji je kreirao/dodelio ovu značku */
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
    /** URL do slike koja se prikazuje na znački */
    displaySrc?: string
    /** Boja pozadine značke (hex kod) */
    backgroundColor?: string
    /** Boja ivice značke (hex kod) */
    borderColor?: string
    /** Boja teksta značke (hex kod) */
    textColor?: string
    /** Dodatna CSS klasa za stilizovanje */
    cssClass?: string
    /** Za veteransku značku, vremenski prag u milisekundama */
    veteranUserThresholdMillis?: number
    /** Da li se ova značka prikazuje na korisnikovim komentarima */
    displayedOnComments: boolean
    /** Redosled prikaza značke */
    order?: number
}
[inline-code-end]
---