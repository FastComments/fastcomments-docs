`UserBadgeProgress` è un oggetto che rappresenta il progresso di un utente verso l'ottenimento di vari badge nel sistema FastComments.

Questo tracciamento aiuta a determinare quando gli utenti dovrebbero ricevere badge automatici in base alla loro attività e partecipazione nella tua community.

La struttura per l'oggetto `UserBadgeProgress` è la seguente:

[inline-code-attrs-start title = 'Struttura di UserBadgeProgress'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Identificatore univoco per questo record di progresso */
    id: string
    /** ID del tenant a cui appartiene questo record di progresso */
    tenantId: string
    /** ID dell'utente che questo record monitora */
    userId: string
    /** ID del primo commento dell'utente nel sistema */
    firstCommentId?: string
    /** Data del primo commento dell'utente (millisecondi dall'epoch) */
    firstCommentDate?: number
    /** Fattore di fiducia calcolato automaticamente in base all'attività dell'utente */
    autoTrustFactor?: number
    /** Fattore di fiducia impostato manualmente dagli amministratori */
    manualTrustFactor?: number
    /** Oggetto di progresso dettagliato con varie metriche, le chiavi corrispondono all'enum BadgeType */
    progress: {
        /** 0: CommentCount - Numero di commenti che l'utente ha pubblicato */
        '0'?: number
        /** 1: CommentUpVotes - Numero di upvote che l'utente ha ricevuto */
        '1'?: number
        /** 2: CommentReplies - Numero di risposte che l'utente ha pubblicato */
        '2'?: number
        /** 3: CommentsPinned - Numero di commenti dell'utente fissati */
        '3'?: number
        /** 4: Veteran - Età dell'account dell'utente */
        '4'?: number
        /** 5: NightOwl - Numero di volte in cui l'utente ha pubblicato durante le ore notturne */
        '5'?: number
        /** 6: FastReplyTime - Tempo medio di risposta in millisecondi */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Per i badge dei moderatori, conteggio dei commenti eliminati */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Per i badge dei moderatori, conteggio dei commenti approvati */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Per i badge dei moderatori, conteggio dei commenti non approvati */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Per i badge dei moderatori, conteggio dei commenti revisionati */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Per i badge dei moderatori, conteggio dei commenti contrassegnati come spam */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Per i badge dei moderatori, conteggio dei commenti contrassegnati come non spam */
        '12'?: number
        /** 13: RepliedToSpecificPage - Per ogni pagina, conteggio delle risposte */
        '13'?: Record<string, number>
    }
}
[inline-code-end]