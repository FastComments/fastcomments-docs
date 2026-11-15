Bei Flex werden Personen, die Ihr Konto verwalten oder moderieren können, als monatliche Seats abgerechnet. Dies ist der Teil einer Flex-Rechnung, der die meisten Menschen am häufigsten überrascht, meist weil Seats erstellt wurden, ohne dass es jemand bemerkt hat.

### Was Als Seat Zählt

- **Admins**: Benutzer auf der `Users`-Seite Ihres Kontos mit einer beliebigen administrativen Berechtigung (Abrechnung, Anpassung, Analysen, Moderation usw.). Der erste Admin, normalerweise der Kontoinhaber, ist kostenlos; jeder weitere Admin ist ein Seat.
- **Moderatoren**: Personen auf der `Moderators`-Seite, die ihre Einladung angenommen haben. Eine ausstehende Einladung wird nicht abgerechnet. Ein Benutzer, dessen einzige Berechtigung die Kommentar-Moderation ist, wird als Moderator abgerechnet, nicht als Admin.
- **SSO-Admins** und **SSO-Moderatoren**: SSO-Benutzer, die sich auf Ihrer Website mit `isAccountOwner`, `isAdminAdmin` oder `isCommentModeratorAdmin` im SSO-Payload anmelden. Sie werden zu denselben Sätzen wie reguläre Admins und Moderatoren abgerechnet. Siehe [SSO User Structure](/guide-api.html#sso-user-structure).

Seats werden anhand der E-Mail-Adresse dedupliziert, sodass eine Person, die sowohl Admin als auch Moderator ist oder sowohl einen SSO-Benutzer als auch einen Dashboard-Login mit derselben E-Mail hat, nur einmal zum höheren Satz abgerechnet wird. Jemand mit zwei verschiedenen E-Mail-Adressen ist zwei Seats.

Bei Konten, die nach monatlich aktiven Benutzern abgerechnet werden, wird ein SSO-Moderator, der sich im Monat nicht angemeldet hat, für diesen Monat nicht abgerechnet.

### Seats, Die Sie Nicht Hinzugefügt Haben

Zwei Funktionen erstellen automatisch Seats:

- **SSO-Admin-Synchronisierung.** Wenn sich ein SSO-Benutzer mit einer E-Mail anmeldet, die mit einem Ihrer Dashboard-Benutzer übereinstimmt, werden die Admin- oder Moderator-Berechtigungen dieses Benutzers auf den SSO-Benutzer kopiert. Bei WordPress-SSO bedeutet dies, dass jeder WordPress-Administrator ein FastComments-Admin oder -Moderator werden kann. Deaktivieren Sie dies unter `Customize` mit der Option `Disable Automatic Admin Migration` in Ihrer Anpassungsregel.
- **Verschiedene E-Mails für dieselbe Person.** Wenn Ihr Dashboard-Login eine E-Mail verwendet und Ihr SSO-Benutzer eine andere, sind dies zwei Seats. Die Verwendung derselben E-Mail für beide führt sie zusammen.

### Überprüfen Und Entfernen Von Seats

Die Seite [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) listet Ihre Anzahl an `Moderators`, `SSO Admins` und `SSO Moderators` auf, und die Vorschau der nächsten Rechnung zeigt deren Kosten. Entfernen Sie nicht benötigte Personen von den Seiten `Users` und `Moderators` oder entfernen Sie die Admin-Flags aus Ihrem SSO-Payload. Seats werden gezählt, wenn die Rechnung am Ende des Monats erstellt wird, sodass ein vor diesem Zeitpunkt entfernter Seat für diesen Monat nicht abgerechnet wird.

Siehe [Adding Moderators](/guide-users.html#adding-moderators) und [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions), um beides zu verwalten.