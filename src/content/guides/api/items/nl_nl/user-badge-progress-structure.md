---
`UserBadgeProgress` is een object dat de voortgang van een gebruiker bijhoudt naar het verdienen van verschillende badges in het FastComments-systeem.

Deze bijhouding helpt bepalen wanneer gebruikers automatisch badges moeten ontvangen op basis van hun activiteit en deelname in uw community.

De structuur van het `UserBadgeProgress`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van UserBadgeProgress'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Unieke identificatie voor dit voortgangsrecord */
    id: string
    /** ID van de tenant waartoe dit voortgangsrecord behoort */
    tenantId: string
    /** ID van de gebruiker die door dit voortgangsrecord wordt gevolgd */
    userId: string
    /** ID van de eerste reactie van de gebruiker in het systeem */
    firstCommentId?: string
    /** Datum van de eerste reactie van de gebruiker (milliseconden sinds epoch) */
    firstCommentDate?: number
    /** Automatisch berekende vertrouwensfactor op basis van gebruikersactiviteit */
    autoTrustFactor?: number
    /** Handmatig ingestelde vertrouwensfactor door beheerders */
    manualTrustFactor?: number
    /** Gedetailleerd voortgangsobject met verschillende statistieken, keys komen overeen met de BadgeType-enum */
    progress: {
        /** 0: CommentCount - Aantal reacties dat de gebruiker heeft geplaatst */
        '0'?: number
        /** 1: CommentUpVotes - Aantal upvotes die de gebruiker heeft ontvangen */
        '1'?: number
        /** 2: CommentReplies - Aantal reacties (replies) die de gebruiker heeft geplaatst */
        '2'?: number
        /** 3: CommentsPinned - Aantal vastgezette reacties die de gebruiker heeft */
        '3'?: number
        /** 4: Veteran - Accountleeftijd van de gebruiker */
        '4'?: number
        /** 5: NightOwl - Aantal keer dat de gebruiker heeft gepost tijdens nachtelijke uren */
        '5'?: number
        /** 6: FastReplyTime - Gemiddelde reactietijd in milliseconden */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Voor moderatorbadges, aantal verwijderde reacties */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Voor moderatorbadges, aantal goedgekeurde reacties */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Voor moderatorbadges, aantal niet-goedgekeurde reacties */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Voor moderatorbadges, aantal beoordeelde reacties */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Voor moderatorbadges, aantal reacties gemarkeerd als spam */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Voor moderatorbadges, aantal reacties gemarkeerd als geen spam */
        '12'?: number
        /** 13: RepliedToSpecificPage - Voor elke pagina, aantal antwoorden */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
---