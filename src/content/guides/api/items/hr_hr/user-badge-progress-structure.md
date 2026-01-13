`UserBadgeProgress` je objekt koji predstavlja napredak korisnika prema stjecanju različitih bedževa u FastComments sustavu.

Ovo praćenje pomaže odrediti kada korisnici trebaju dobiti automatske bedževe na temelju njihove aktivnosti i sudjelovanja u vašoj zajednici.

Struktura za `UserBadgeProgress` objekt je sljedeća:

[inline-code-attrs-start title = 'Struktura UserBadgeProgress objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Jedinstveni identifikator ovog zapisa napretka */
    id: string
    /** ID tenanta kojem pripada ovaj zapis napretka */
    tenantId: string
    /** ID korisnika kojeg ovaj zapis napretka prati */
    userId: string
    /** ID prvog komentara korisnika u sustavu */
    firstCommentId?: string
    /** Datum prvog komentara korisnika (milisekunde od epohe) */
    firstCommentDate?: number
    /** Automatski izračunati faktor povjerenja na temelju aktivnosti korisnika */
    autoTrustFactor?: number
    /** Faktor povjerenja postavljen ručno od strane administratora */
    manualTrustFactor?: number
    /** Detaljan objekt napretka s različitim metrima, ključevi odgovaraju BadgeType enumeraciji */
    progress: {
        /** 0: CommentCount - Broj komentara koje je korisnik napisao */
        '0'?: number
        /** 1: CommentUpVotes - Broj pozitivnih glasova (upvote) koje je korisnik primio */
        '1'?: number
        /** 2: CommentReplies - Broj odgovora koje je korisnik napisao */
        '2'?: number
        /** 3: CommentsPinned - Broj prikvačenih (pinned) komentara koje korisnik ima */
        '3'?: number
        /** 4: Veteran - Starost korisničkog računa */
        '4'?: number
        /** 5: NightOwl - Broj puta kada je korisnik objavio tijekom noćnih sati */
        '5'?: number
        /** 6: FastReplyTime - Prosječno vrijeme odgovora u milisekundama */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Za moderator bedževe, broj izbrisanih komentara */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Za moderator bedževe, broj odobrenih komentara */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Za moderator bedževe, broj neodobrenih komentara */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Za moderator bedževe, broj pregledanih komentara */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Za moderator bedževe, broj komentara označenih kao spam */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Za moderator bedževe, broj komentara označenih kao ne-spam */
        '12'?: number
        /** 13: RepliedToSpecificPage - Za svaku stranicu, broj odgovora */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
---