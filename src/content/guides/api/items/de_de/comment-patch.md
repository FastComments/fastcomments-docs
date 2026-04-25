[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, einen einzelnen Kommentar zu aktualisieren.

Hinweise:

- Diese API kann das Kommentar-Widget "live" aktualisieren, falls gewünscht (dies erhöht die Grund-`creditsCost` von `1` auf `2`).
  - Dadurch kann das Migrieren von Kommentaren zwischen Seiten "live" erfolgen (Änderung von `urlId`).
  - Migrationen kosten zusätzliche `2` Credits, da Seiten vorab berechnet werden und dies CPU-intensiv ist.
- Im Gegensatz zur create API erstellt diese API NICHT automatisch Benutzerobjekte in unserem System, wenn eine E-Mail angegeben wird.
- Über diese API aktualisierte Kommentare können bei Bedarf weiterhin auf Spam geprüft werden.
- Konfigurationen wie die maximale Kommentarlänge, sofern über die Customization Rule Admin-Seite konfiguriert, gelten hier.
- Um Benutzern zu ermöglichen, ihren Kommentartext zu aktualisieren, können Sie einfach `comment` im Request-Body angeben. Wir generieren das resultierende `commentHTML`.
  - Wenn Sie sowohl `comment` als auch `commentHTML` definieren, generieren wir das HTML nicht automatisch.
  - Wenn der Benutzer in seinem neuen Text Erwähnungen oder Hashtags hinzufügt, werden diese weiterhin wie bei der `POST` API verarbeitet.
- Beim Aktualisieren von `commenterEmail` an einem Kommentar ist es ratsam, auch `userId` anzugeben. Andernfalls müssen Sie sicherstellen, dass der Benutzer mit dieser E-Mail zu Ihrem Tenant gehört, sonst schlägt die Anfrage fehl.  
- Wenn der Zielkommentar gesperrt ist (`isLocked: true`), wird die Anfrage mit `code: 'locked'` abgelehnt. Entsperren Sie zuerst den Kommentar, aktualisieren Sie ihn und sperren Sie ihn anschließend bei Bedarf wieder.


[inline-code-attrs-start title = 'Minimales Kommentar-PATCH cURL-Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Kommentar-PATCH-Anfrage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Der Benutzer, der das Update durchführt. Kann optional verwendet werden, um zu prüfen, ob er den Kommentar bearbeiten darf.  **/
    contextUserId?: string
	/** Sollen wir prüfen, ob der neue Kommentar nach Spam aussieht?  **/
    doSpamCheck?: 'true' | 'false'
	/** Ob der Kommentar für Benutzer, die Instanzen des Kommentar-Widgets mit derselben urlId ansehen, als "live" erscheinen soll. HINWEIS: Verdoppelt die Kreditkosten von 1 auf 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Kommentar-PATCH-Antwort'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Bei Fehlschlag enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Bei Fehlschlag enthalten. **/
    reason?: string
}
[inline-code-end]