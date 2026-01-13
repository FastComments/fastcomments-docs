FastComments funktioniert mit Mitgliedschaftsseiten, indem es SSO, also Single Sign-On, verwendet. Ihre Mitglieder melden sich auf Ihrer WordPress-Seite an, aber
müssen sich nicht um die Erstellung eines Kontos bei FastComments oder die Anmeldung mit sozialen Medien kümmern, um zu kommentieren. Wenn Ihre Mitglieder (einschließlich Administratoren) auf Ihrer WordPress-Seite eingeloggt sind,
können sie kommentieren. Ihre Administratoren und Moderatoren können Kommentare direkt aus Ihren WordPress-Blogbeiträgen moderieren.

<sup>(Optional)</sup> Denken Sie auch daran, Ihre Administratoren zu [Benutzer & Administratoren](https://fastcomments.com/auth/my-account/users) und Moderatoren zu [Kommentar-Moderatoren](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
hinzuzufügen, um deren Erlebnis zu verbessern und die Statistikverfolgung für Moderatoren zu aktivieren.

SSO kann aktiviert werden, indem Sie zum Plugin-Dashboard gehen und auf "SSO Settings" klicken.

Sie müssen zunächst die Funktion "Anyone can Register" auf Ihrer Seite aktivieren.

Alle Benutzerinformationen werden nahtlos und sicher an FastComments übertragen, jedes Mal, wenn ein Benutzer einen Kommentar-Thread über [HMAC](https://en.wikipedia.org/wiki/HMAC) ansieht.

Es ist keine anfängliche oder kontinuierliche Synchronisation nötig, um Ihre Mitglieder auf die FastComments-Server zu kopieren. Dies geschieht automatisch, wenn sie Kommentar-Threads sehen, indem sie einen Blogbeitrag öffnen.

## Namen und Avatare

Das Plugin aktualisiert automatisch den Anzeigenamen und das Avatar des Benutzers bei all seinen Kommentaren in FastComments, wenn sie
einen Kommentar-Thread ansehen. Avatare werden über Gravatar oder jedes Avatar-Verwaltungs-Plugin innerhalb von WordPress unterstützt, da das Plugin `get_avatar_url` aufrufen wird.