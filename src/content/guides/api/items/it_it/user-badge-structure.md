`UserBadge` è un oggetto che rappresenta un badge assegnato a un utente nel sistema FastComments.

I badge possono essere assegnati agli utenti automaticamente in base alla loro attività (come il numero di commenti, il tempo di risposta, lo stato di veterano) o manualmente dagli amministratori del sito.

La struttura dell'oggetto `UserBadge` è la seguente:

[inline-code-attrs-start title = 'Struttura di UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Identificatore univoco per questa assegnazione del badge all'utente */
    id: string
    /** ID dell'utente a cui è assegnato questo badge */
    userId: string
    /** ID della definizione del badge dal catalogo dei badge del tenant */
    badgeId: string
    /** ID del tenant che ha creato/assegnato questo badge */
    fromTenantId: string
    /** Quando è stato creato questo badge (millisecondi dall'epoch) */
    createdAt?: number
    /** Quando questo badge è stato ricevuto dall'utente (millisecondi dall'epoch) */
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
    /** Per i badge basati su soglia, il valore della soglia */
    threshold?: number
    /** Il nome/etichetta del badge */
    name?: string
    /** Descrizione dettagliata del badge */
    description?: string
    /** Testo mostrato sul badge */
    displayLabel?: string
    /** URL di un'immagine mostrata sul badge */
    displaySrc?: string
    /** Colore di sfondo del badge (codice esadecimale) */
    backgroundColor?: string
    /** Colore del bordo del badge (codice esadecimale) */
    borderColor?: string
    /** Colore del testo del badge (codice esadecimale) */
    textColor?: string
    /** Classe CSS aggiuntiva per lo styling */
    cssClass?: string
    /** Per i badge veterano, la soglia temporale in millisecondi */
    veteranUserThresholdMillis?: number
    /** Se questo badge è visualizzato nei commenti dell'utente */
    displayedOnComments: boolean
    /** L'ordine di visualizzazione del badge */
    order?: number
    /** Se impostato, questo badge è mostrato solo nella pagina con urlId corrispondente. Null per i badge globali. */
    urlId?: string | null
}
[inline-code-end]