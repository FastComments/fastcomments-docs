`UserBadge` è un oggetto che rappresenta un badge assegnato a un utente nel sistema FastComments.

I badge possono essere assegnati agli utenti automaticamente in base alla loro attività (ad esempio conteggio dei commenti, tempo di risposta, stato di veterano) o manualmente dagli amministratori del sito.

La struttura per l'oggetto `UserBadge` è la seguente:

[inline-code-attrs-start title = 'Struttura di UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Identificatore univoco per questa assegnazione di badge all'utente */
    id: string
    /** ID dell'utente a cui è assegnato questo badge */
    userId: string
    /** ID della definizione del badge dal catalogo badge del tenant */
    badgeId: string
    /** ID del tenant che ha creato/assegnato questo badge */
    fromTenantId: string
    /** Quando questo badge è stato creato (millisecondi dall'epoca) */
    createdAt?: number
    /** Quando questo badge è stato ricevuto dall'utente (millisecondi dall'epoca) */
    receivedAt?: number
    /** 
     * Il tipo di badge: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Per badge basati su soglia, il valore della soglia */
    threshold?: number
    /** Il nome/etichetta del badge */
    name?: string
    /** Descrizione dettagliata del badge */
    description?: string
    /** Il testo mostrato sul badge */
    displayLabel?: string
    /** URL dell'immagine mostrata sul badge */
    displaySrc?: string
    /** Colore di sfondo per il badge (codice esadecimale) */
    backgroundColor?: string
    /** Colore del bordo per il badge (codice esadecimale) */
    borderColor?: string
    /** Colore del testo per il badge (codice esadecimale) */
    textColor?: string
    /** Classe CSS aggiuntiva per lo styling */
    cssClass?: string
    /** Per i badge veterano, la soglia temporale in millisecondi */
    veteranUserThresholdMillis?: number
    /** Se questo badge viene visualizzato nei commenti dell'utente */
    displayedOnComments: boolean
    /** L'ordine di visualizzazione del badge */
    order?: number
}
[inline-code-end]
---