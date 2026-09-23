[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Gefällt einer Seite als aktueller Benutzer. Jeder Benutzer kann einer Seite nur einmal ein Like geben: Ein erneutes Liken schlägt mit dem Code `already-liked` erfolgreich durch und ändert die Zählung nicht.

Die Seite wird erstellt, falls sie noch nicht existiert. Übergeben Sie `title`, um den Titel der Seite festzulegen oder zu aktualisieren.

[inline-code-attrs-start title = 'Beispiel für Page Like cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Page Like-Anfrage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Setzt den Titel der Seite. **/
    title?: string
    /** URI-codiertes JSON Ihres SSO-Objekts. Für anonyme Benutzer weglassen. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Page Like-Antwort'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked', wenn der Benutzer die Seite bereits geliked hat. Andernfalls bei einem Fehler enthalten. **/
    code?: 'already-liked' | string
    /** Bei einem Fehler enthalten. **/
    reason?: string
}
[inline-code-end]