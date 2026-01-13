Ein `Moderator`-Objekt repräsentiert die Konfiguration für einen Moderator.

Es gibt drei Arten von Moderatoren:

1. Administratorbenutzer, die das `isCommentModeratorAdmin`-Flag haben.
2. SSO-Benutzer mit dem `isCommentModeratorAdmin`-Flag.
3. Reguläre Kommentatoren oder FastComments.com-Benutzer, die als Moderatoren eingeladen werden.

Die `Moderator`-Struktur wird verwendet, um den Moderationsstatus von Anwendungsfall `3` darzustellen.

Wenn Sie einen Benutzer über die API als Moderator einladen möchten, verwenden Sie die `Moderator`-API, indem Sie einen `Moderator` erstellen und ihn `einladen`.

Wenn der Benutzer kein FastComments.com-Konto hat, hilft ihm die Einladungs-E-Mail bei der Einrichtung. Wenn er bereits ein Konto hat, erhält er
Moderationszugriff auf Ihren Tenant und die `userId` des `Moderator`-Objekts wird aktualisiert, um auf seinen Benutzer zu verweisen. Sie werden keinen API-Zugriff
auf seinen Benutzer haben, da dieser in diesem Fall ihm selbst gehört und von FastComments.com verwaltet wird.

Wenn Sie eine vollständige Verwaltung des Benutzerkontos benötigen, empfehlen wir entweder die Verwendung von SSO oder das Hinzufügen als [Tenant-Benutzer](https://fastcomments.com/auth/my-account/users) und
dann das Hinzufügen eines `Moderator`-Objekts, um ihre Statistiken zu verfolgen.

Die `Moderator`-Struktur kann als Statistik-Verfolgungsmechanismus für die Anwendungsfälle `1` und `2` verwendet werden. Nachdem Sie den Benutzer erstellt haben, fügen Sie ein `Moderator`-Objekt
mit definierter `userId` hinzu und ihre Statistiken werden auf der [Kommentar-Moderatoren-Seite](https://fastcomments.com/auth/my-account/moderate-comments/moderators) verfolgt.

Die Struktur des `Moderator`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'Moderator Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]
