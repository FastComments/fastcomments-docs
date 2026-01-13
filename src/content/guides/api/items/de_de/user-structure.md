`User` ist ein Objekt, das den kleinsten gemeinsamen Nenner aller Benutzer darstellt.

Beachten Sie, dass wir bei FastComments verschiedene Anwendungsfälle für Benutzer haben:

- Secure SSO
- Simple SSO
- Tenant-Benutzer (zum Beispiel: Administratoren)
- Kommentierende

Diese API ist für **Kommentierende** und Benutzer, die über **Simple SSO** erstellt wurden. Grundsätzlich kann über diese API auf jeden Benutzer zugegriffen werden, der
über Ihre Seite erstellt wurde. Tenant-Benutzer können auch auf diese Weise abgerufen werden, aber Sie erhalten mehr Informationen durch Interaktion mit der `/tenant-users/`-API.

Für `Secure SSO` verwenden Sie bitte die `/sso-users/`-API.

Sie können diese Arten von Benutzern nicht aktualisieren. Sie haben ihr Konto über Ihre Seite erstellt, daher bieten wir einen grundlegenden schreibgeschützten Zugang, aber
Sie können keine Änderungen vornehmen. Wenn Sie diesen Flow haben möchten, müssen Sie `Secure SSO` einrichten.

Die Struktur des `User`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'User Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** This is also the id used as userId on comment objects. **/
    id: string
    username: string
    /** A link to the commenter's blog, for example. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]
