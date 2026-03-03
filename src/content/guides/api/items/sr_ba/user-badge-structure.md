`UserBadge` je objekat koji predstavlja značku dodijeljenu korisniku u FastComments sistemu.

Značke se mogu automatski dodijeliti korisnicima na osnovu njihove aktivnosti (takve kao broj komentara, vrijeme odgovora, status veterana) ili ručno od strane administratora sajta.

Struktura za `UserBadge` objekat je sljedeća:

[inline-code-attrs-start title = 'Struktura objekta UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
     * Tip značke: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Za značke zasnovane na pragu, vrijednost praga */
    threshold?: number
    /** Naziv/oznaka značke */
    name?: string
    /** Detaljan opis značke */
    description?: string
    /** Tekst koji se prikazuje na znački */
    displayLabel?: string
    /** URL slike prikazane na znački */
    displaySrc?: string
    /** Boja pozadine značke (hex kod) */
    backgroundColor?: string
    /** Boja okvira značke (hex kod) */
    borderColor?: string
    /** Boja teksta značke (hex kod) */
    textColor?: string
    /** Dodatna CSS klasa za stilizovanje */
    cssClass?: string
    /** Za veteranske značke, vremenski prag u milisekundama */
    veteranUserThresholdMillis?: number
    /** Da li se ova značka prikazuje na korisnikovim komentarima */
    displayedOnComments: boolean
    /** Redoslijed prikaza značke */
    order?: number
    /** Ako je postavljeno, ova značka se prikazuje samo na stranici sa odgovarajućim urlId. Null za globalne značke. */
    urlId?: string | null
}
[inline-code-end]