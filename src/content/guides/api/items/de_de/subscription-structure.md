Ein `Subscription`-Objekt repräsentiert ein Abonnement für einen Benutzer.

`Subscription`-Objekte werden erstellt, wenn ein Benutzer auf das Benachrichtigungsglocken-Symbol im Kommentar-Widget klickt und "Diese Seite abonnieren" auswählt.

Abonnements können auch über die API erstellt werden.

Das Vorhandensein eines `Subscription`-Objekts bewirkt, dass `Notification`-Objekte generiert und E-Mails gesendet werden, wenn neue Kommentare am Wurzelknoten der zugehörigen Seite hinterlassen werden,
für die das `Subscription` gilt. Das Senden von E-Mails hängt vom Benutzertyp ab. Für reguläre Benutzer hängt dies von `optedInNotifications` ab. Für SSO-Benutzer hängt dies von `optedInSubscriptionNotifications` ab. Beachten Sie, dass einige Anwendungen möglicherweise kein Konzept einer webzugänglichen Seite haben, in diesem Fall setzen Sie `urlId` einfach auf
die ID des Elements, das Sie abonnieren (derselbe Wert für `urlId`, den Sie an das Kommentar-Widget übergeben würden).

Die Struktur des `Subscription`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'Abonnement Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]
