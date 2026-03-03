`UserBadge` ist ein Objekt, das ein Abzeichen repräsentiert, das einem Benutzer im FastComments-System zugewiesen ist.

Abzeichen können Benutzern automatisch aufgrund ihrer Aktivität (z. B. Anzahl der Kommentare, Antwortzeit, Veteranenstatus) oder manuell von Website-Administratoren zugewiesen werden.

Die Struktur des `UserBadge`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'UserBadge-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Eindeutige Kennung für diese Zuordnung des Benutzerabzeichens */
    id: string
    /** ID des Benutzers, dem dieses Abzeichen zugewiesen ist */
    userId: string
    /** ID der Abzeichendefinition aus dem Abzeichenkatalog des Mandanten */
    badgeId: string
    /** ID des Mandanten, der dieses Abzeichen erstellt/zugewiesen hat */
    fromTenantId: string
    /** Wann dieses Abzeichen erstellt wurde (Millisekunden seit Epoch) */
    createdAt?: number
    /** Wann dieses Abzeichen vom Benutzer erhalten wurde (Millisekunden seit Epoch) */
    receivedAt?: number
    /** 
     * Der Abzeichentyp: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Für schwellenbasierte Abzeichen der Schwellenwert */
    threshold?: number
    /** Der Name/Bezeichner des Abzeichens */
    name?: string
    /** Detaillierte Beschreibung des Abzeichens */
    description?: string
    /** Der auf dem Abzeichen angezeigte Text */
    displayLabel?: string
    /** URL zu einem auf dem Abzeichen angezeigten Bild */
    displaySrc?: string
    /** Hintergrundfarbe des Abzeichens (Hex-Code) */
    backgroundColor?: string
    /** Randfarbe des Abzeichens (Hex-Code) */
    borderColor?: string
    /** Textfarbe des Abzeichens (Hex-Code) */
    textColor?: string
    /** Zusätzliche CSS-Klasse zur Gestaltung */
    cssClass?: string
    /** Für Veteranen-Abzeichen die Zeitgrenze in Millisekunden */
    veteranUserThresholdMillis?: number
    /** Ob dieses Abzeichen in den Kommentaren des Benutzers angezeigt wird */
    displayedOnComments: boolean
    /** Die Anzeigereihenfolge des Abzeichens */
    order?: number
    /** Falls gesetzt, wird dieses Abzeichen nur auf der Seite mit der passenden urlId angezeigt. Null für globale Abzeichen. */
    urlId?: string | null
}
[inline-code-end]