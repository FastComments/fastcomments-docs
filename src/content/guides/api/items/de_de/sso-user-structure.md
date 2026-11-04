FastComments bietet eine benutzerfreundliche SSO-Lösung. Die Aktualisierung der Informationen eines Benutzers mit der HMAC-basierten Integration ist so einfach, dass der Benutzer nur eine Seite mit einem aktualisierten Payload laden muss.

Es kann jedoch wünschenswert sein, einen Benutzer außerhalb dieses Ablaufs zu verwalten, um die Konsistenz Ihrer Anwendung zu verbessern.

Die SSO User API bietet eine Möglichkeit, Objekte zu verwalten (CRUD), die wir SSOUsers nennen. Diese Objekte unterscheiden sich von regulären Benutzern und werden zur Typsicherheit getrennt gehalten.

Die Struktur des SSOUser-Objekts ist wie folgt:

[inline-code-attrs-start title = 'SSOUser-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Admin-Berechtigung - SSO-Benutzer mit diesem Flag werden als SSO-Admins abgerechnet (getrennt von regulären SSO-Benutzern)
    isAdminAdmin?: boolean // Admin-Berechtigung - SSO-Benutzer mit diesem Flag werden als SSO-Admins abgerechnet (getrennt von regulären SSO-Benutzern)
    isCommentModeratorAdmin?: boolean // Moderator-Berechtigung - SSO-Benutzer mit diesem Flag werden als SSO-Moderatoren abgerechnet (getrennt von regulären SSO-Benutzern)
    /** Wenn null, wird keine Zugriffssteuerung auf den Benutzer angewendet. Bei einer leeren Liste kann dieser Benutzer keine Seiten sehen und keine anderen Benutzer per @mention erwähnen. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Verhindert, dass andere Benutzer die Aktivitäten dieses Benutzers, einschließlich Kommentaren, in seinem Profil sehen. Standard ist true, um standardmäßig sichere Profile bereitzustellen. **/
    isProfileActivityPrivate?: boolean
    /** Verhindert, dass andere Benutzer Kommentare im Profil des Benutzers hinterlassen oder vorhandene Profilkommentare sehen. Standard ist false. **/
    isProfileCommentsPrivate?: boolean
    /** Verhindert, dass andere Benutzer diesem Benutzer direkte Nachrichten senden. Standard ist false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optionale Konfiguration für Benutzerabzeichen. **/
    badgeConfig?: {
        /** Array von Badge-IDs, die dem Benutzer zugewiesen werden sollen. Auf 30 Badges begrenzt. Die Reihenfolge wird beachtet. Dies sind globale Badges, die auf allen Seiten sichtbar sind. **/
        badgeIds: string[]
        /** Array von Badge-IDs, die auf die aktuelle Seite (urlId) beschränkt sind. Diese Badges werden nur auf der Seite angezeigt, auf der sie zugewiesen wurden. **/
        pageBadgeIds?: string[]
        /** Wenn true, werden alle vorhandenen angezeigten Badges durch die angegebenen ersetzt. Globale und seitenspezifische Badges werden unabhängig voneinander überschrieben. Wenn false, werden die Badges zu den vorhandenen hinzugefügt. **/
        override?: boolean
        /** Wenn true, werden die Anzeigeeigenschaften der Badges aus der Tenant-Konfiguration aktualisiert. **/
        update?: boolean
    }
}
[inline-code-end]

### Abrechnung für SSO-Benutzer

SSO-Benutzer werden je nach ihren Berechtigungs-Flags unterschiedlich abgerechnet:

- **Reguläre SSO-Benutzer**: Benutzer ohne Admin- oder Moderator-Berechtigungen werden als reguläre SSO-Benutzer abgerechnet
- **SSO-Admins**: Benutzer mit den Flags `isAccountOwner` oder `isAdminAdmin` werden separat als SSO-Admins abgerechnet (gleicher Satz wie reguläre Tenant-Admins)
- **SSO-Moderatoren**: Benutzer mit dem Flag `isCommentModeratorAdmin` werden separat als SSO-Moderatoren abgerechnet (gleicher Satz wie reguläre Moderatoren)

**Wichtig**: Um eine doppelte Abrechnung zu verhindern, dedupliziert das System SSO-Benutzer automatisch anhand der E-Mail-Adresse gegen reguläre Tenant-Benutzer und Moderatoren. Wenn ein SSO-Benutzer dieselbe E-Mail-Adresse wie ein regulärer Tenant-Benutzer oder Moderator hat, wird er nicht doppelt abgerechnet.

### Zugriffssteuerung

Benutzer können in Gruppen aufgeteilt werden. Dafür ist das Feld `groupIds` vorgesehen, das optional ist.

### @Erwähnungen

Standardmäßig verwendet die `@mentions`-Funktion `username`, um nach anderen SSO-Benutzern zu suchen, wenn das Zeichen `@` eingegeben wird. Wenn `displayName` verwendet wird, werden Ergebnisse, die `username` entsprechen, ignoriert, wenn es eine Übereinstimmung für `displayName` gibt, und die `@mention`-Suchergebnisse verwenden `displayName`.

### Abonnements

Mit FastComments können Benutzer eine Seite abonnieren, indem sie auf das Glockensymbol im Kommentar-Widget klicken und auf Abonnieren klicken.

Bei einem regulären Benutzer senden wir ihm Benachrichtigungs-E-Mails basierend auf seinen Benachrichtigungseinstellungen.

Bei SSO-Benutzern haben wir dies aus Gründen der Abwärtskompatibilität aufgeteilt. Benutzer erhalten diese zusätzlichen Abonnement-Benachrichtigungs-E-Mails nur, wenn Sie `optedInSubscriptionNotifications` auf `true` setzen.

### Badges

Sie können SSO-Benutzern Badges über die Eigenschaft `badgeConfig` zuweisen. Badges sind visuelle Kennzeichnungen, die neben dem Namen eines Benutzers in Kommentaren erscheinen.

- `badgeIds` - Ein Array von Badge-IDs, die dem Benutzer zugewiesen werden sollen. Dies sind globale Badges, die auf allen Seiten sichtbar sind. Muss gültige Badge-IDs enthalten, die in Ihrem FastComments-Konto erstellt wurden. Auf 30 Badges begrenzt.
- `pageBadgeIds` - Ein optionales Array von Badge-IDs, die auf die aktuelle Seite (`urlId`) beschränkt sind. Diese Badges werden nur auf der Seite angezeigt, auf der sie zugewiesen wurden. Verschiedene Seiten können unterschiedliche seitenspezifische Badges für denselben Benutzer haben.
- `override` - Wenn true, werden alle vorhandenen angezeigten Badges durch die angegebenen ersetzt. Globale und seitenspezifische Badges werden unabhängig voneinander überschrieben — das Überschreiben globaler Badges wirkt sich nicht auf seitenspezifische Badges aus und umgekehrt. Wenn false oder weggelassen, werden die angegebenen Badges zu den vorhandenen hinzugefügt.
- `update` - Wenn true, werden die Anzeigeeigenschaften der Badges bei jeder Anmeldung des Benutzers aus der Tenant-Konfiguration aktualisiert.