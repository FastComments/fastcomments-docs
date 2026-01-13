---
`UserBadgeProgress` je objekat koji predstavlja napredak korisnika u sticanju različitih znački u FastComments sistemu.

Ovo praćenje pomaže da se odredi kada korisnici treba da dobiju automatske značke na osnovu njihove aktivnosti i učešća u vašoj zajednici.

Struktura `UserBadgeProgress` objekta je sledeća:

[inline-code-attrs-start title = 'Struktura UserBadgeProgress objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Jedinstveni identifikator ovog zapisa o napretku */
    id: string
    /** ID tenanta kome ovaj zapis o napretku pripada */
    tenantId: string
    /** ID korisnika koga ovaj zapis o napretku prati */
    userId: string
    /** ID prvog komentara korisnika u sistemu */
    firstCommentId?: string
    /** Datum prvog komentara korisnika (milisekunde od epohe) */
    firstCommentDate?: number
    /** Automatski izračunat faktor poverenja na osnovu aktivnosti korisnika */
    autoTrustFactor?: number
    /** Ručno postavljen faktor poverenja od strane administratora */
    manualTrustFactor?: number
    /** Detaljan objekat napretka sa različitim metrikama, ključevi odgovaraju BadgeType enum-u */
    progress: {
        /** 0: CommentCount - Broj komentara koje je korisnik ostavio */
        '0'?: number
        /** 1: CommentUpVotes - Broj pozitivnih glasova koje je korisnik dobio */
        '1'?: number
        /** 2: CommentReplies - Broj odgovora koje je korisnik postavio */
        '2'?: number
        /** 3: CommentsPinned - Broj prikačenih komentara koje korisnik ima */
        '3'?: number
        /** 4: Veteran - Starost naloga korisnika */
        '4'?: number
        /** 5: NightOwl - Broj puta kada je korisnik objavio tokom noćnih sati */
        '5'?: number
        /** 6: FastReplyTime - Prosečno vreme odgovora u milisekundama */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Za moderatorške značke, broj obrisanih komentara */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Za moderatorške značke, broj odobrenih komentara */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Za moderatorške značke, broj neodobrenih komentara */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Za moderatorške značke, broj pregledanih komentara */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Za moderatorške značke, broj komentara označenih kao spam */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Za moderatorške značke, broj komentara označenih kao ne-spam */
        '12'?: number
        /** 13: RepliedToSpecificPage - Za svaku stranicu, broj odgovora */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
---