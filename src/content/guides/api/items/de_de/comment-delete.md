[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, einen Kommentar zu löschen.

Hinweise:

- Diese API kann das Kommentar-Widget bei Bedarf "live" aktualisieren (dies erhöht die `creditsCost` von `1` auf `2`).
- Diese API löscht alle untergeordneten Kommentare.
- Wenn der Zielkommentar gesperrt ist (`isLocked: true`), wird die Anfrage mit `code: 'locked'` abgelehnt. Entsperren Sie zuerst den Kommentar, und löschen Sie ihn dann.

[inline-code-attrs-start title = 'Kommentar DELETE cURL-Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Kommentar DELETE Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Der Benutzer, der das Update durchführt. Kann optional verwendet werden, um zu prüfen, ob er den Kommentar löschen darf.  **/
    contextUserId?: string
	/** Ob der Kommentar "live" für Benutzer gelöscht werden soll, die Instanzen des Kommentar-Widgets mit derselben urlId ansehen. HINWEIS: Verdoppelt die Kreditkosten von 1 auf 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Kommentar DELETE Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Bei Fehlschlag enthalten. **/
    reason?: string
}
[inline-code-end]