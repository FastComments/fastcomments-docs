`UserBadgeProgress` je objekt, ki predstavlja napredek uporabnika pri pridobivanju različnih značk v sistemu FastComments.

To sledenje pomaga določiti, kdaj naj uporabniki prejmejo samodejne značke glede na njihovo dejavnost in sodelovanje v vaši skupnosti.

Struktura objekta `UserBadgeProgress` je naslednja:

[inline-code-attrs-start title = 'Struktura UserBadgeProgress'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Enolični identifikator tega zapisa o napredku */
    id: string
    /** ID najemnika, kateremu pripada ta zapis o napredku */
    tenantId: string
    /** ID uporabnika, katerega ta zapis o napredku spremlja */
    userId: string
    /** ID prvega komentarja uporabnika v sistemu */
    firstCommentId?: string
    /** Datum prvega komentarja uporabnika (milisekunde od epohe) */
    firstCommentDate?: number
    /** Samodejno izračunan faktor zaupanja na podlagi dejavnosti uporabnika */
    autoTrustFactor?: number
    /** Ročno nastavljen faktor zaupanja s strani skrbnikov */
    manualTrustFactor?: number
    /** Podroben objekt napredka z različnimi metrikami, ključi ustrezajo BadgeType enum */
    progress: {
        /** 0: CommentCount - Število komentarjev, ki jih je uporabnik napisal */
        '0'?: number
        /** 1: CommentUpVotes - Število všečkov, ki jih je uporabnik prejel */
        '1'?: number
        /** 2: CommentReplies - Število odgovorov, ki jih je uporabnik napisal */
        '2'?: number
        /** 3: CommentsPinned - Število pripetih komentarjev, ki jih ima uporabnik */
        '3'?: number
        /** 4: Veteran - Starost uporabniškega računa */
        '4'?: number
        /** 5: NightOwl - Število objav uporabnika med nočnimi urami */
        '5'?: number
        /** 6: FastReplyTime - Povprečni čas odgovora v milisekundah */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Za moderatorjske značke, število izbrisanih komentarjev */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Za moderatorjske značke, število odobrenih komentarjev */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Za moderatorjske značke, število neodobrenih komentarjev */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Za moderatorjske značke, število pregledanih komentarjev */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Za moderatorjske značke, število komentarjev označenih kot spam */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Za moderatorjske značke, število komentarjev označenih kot ne-spam */
        '12'?: number
        /** 13: RepliedToSpecificPage - Za vsako stran, število odgovorov */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
---