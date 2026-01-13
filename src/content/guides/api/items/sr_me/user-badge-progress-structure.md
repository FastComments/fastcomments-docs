`UserBadgeProgress` je objekat koji predstavlja napredak korisnika ka sticanju različitih znački u FastComments sistemu.

Ovo praćenje pomaže da se utvrdi kada korisnici treba da dobiju automatske značke na osnovu njihove aktivnosti i učešća u vašoj zajednici.

Struktura `UserBadgeProgress` objekta je sljedeća:

[inline-code-attrs-start title = 'Struktura UserBadgeProgress objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Jedinstveni identifikator ovog zapisa o napretku */
    id: string
    /** ID tenant-a kome ovaj zapis o napretku pripada */
    tenantId: string
    /** ID korisnika kojeg ovaj zapis o napretku prati */
    userId: string
    /** ID prvog komentara korisnika u sistemu */
    firstCommentId?: string
    /** Datum prvog komentara korisnika (milisekunde od Unix epohe) */
    firstCommentDate?: number
    /** Automatski izračunat faktor povjerenja zasnovan na aktivnosti korisnika */
    autoTrustFactor?: number
    /** Faktor povjerenja ručno postavljen od strane administratora */
    manualTrustFactor?: number
    /** Detaljan objekat napretka sa raznim metrikama, ključevi odgovaraju BadgeType enumeraciji */
    progress: {
        /** 0: CommentCount - Broj komentara koje je korisnik ostavio */
        '0'?: number
        /** 1: CommentUpVotes - Broj upvota koje je korisnik dobio */
        '1'?: number
        /** 2: CommentReplies - Broj odgovora koje je korisnik dao */
        '2'?: number
        /** 3: CommentsPinned - Broj pinovanih komentara koje korisnik ima */
        '3'?: number
        /** 4: Veteran - Starost korisničkog naloga */
        '4'?: number
        /** 5: NightOwl - Broj puta kada je korisnik objavio tokom noćnih sati */
        '5'?: number
        /** 6: FastReplyTime - Prosječno vrijeme odgovora u milisekundama */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Za moderatorne značke, broj izbrisanih komentara */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Za moderatorne značke, broj odobrenih komentara */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Za moderatorne značke, broj neodobrenih komentara */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Za moderatorne značke, broj pregledanih komentara */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Za moderatorne značke, broj komentara označenih kao spam */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Za moderatorne značke, broj komentara označenih kao 'nije spam' */
        '12'?: number
        /** 13: RepliedToSpecificPage - Za svaku stranicu, broj odgovora */
        '13'?: Record<string, number>
    }
}
[inline-code-end]