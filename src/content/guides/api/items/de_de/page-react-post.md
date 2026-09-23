[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Fügt einer Seite als aktueller Benutzer eine Reaktion hinzu. Ein Benutzer kann jede Reaktions-ID nur einmal hinzufügen: Ein erneutes Hinzufügen schlägt mit dem Code `already-reacted` fehl und ändert die Zählung nicht. Ein Benutzer kann mehrere verschiedene Reaktionen zur selben Seite hinzufügen.

Reaktions-IDs werden von Ihnen gewählt und können bis zu 36 Zeichen lang sein. Die Seite wird erstellt, falls sie noch nicht existiert. Übergeben Sie `title`, um den Titel der Seite zu setzen oder zu aktualisieren.

[inline-code-attrs-start title = 'Page React cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Page React Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** Die Reaktions-ID, bis zu 36 Zeichen. **/
    id: string
    /** Setzt den Titel der Seite. **/
    title?: string
    /** URI-codiertes JSON Ihres SSO-Objekts. Für anonyme Benutzer weglassen. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page React Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted', wenn der Benutzer diese Reaktion bereits hinzugefügt hat. 'react-id-too-long' (HTTP 422), wenn die ID länger als 36 Zeichen ist. Andernfalls bei einem Fehler enthalten. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Bei einem Fehler enthalten. **/
    reason?: string
}
[inline-code-end]

---