[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, einen einzelnen Kommentar zu aktualisieren.

Hinweise:

- Diese API kann das Kommentar-Widget bei Bedarf "live" aktualisieren (dies erhöht die Basis-`creditsCost` von `1` auf `2`).
  - Dies kann das Migrieren von Kommentaren zwischen Seiten "live" machen (Ändern der `urlId`).
  - Migrationen kosten zusätzlich `2` Credits, da Seiten vorberechnet werden und dies CPU-intensiv ist.
- Anders als bei der Create-API wird diese API NICHT automatisch Benutzerobjekte in unserem System erstellen, wenn eine E-Mail angegeben wird.
- Über diese API aktualisierte Kommentare können bei Bedarf immer noch auf Spam überprüft werden.
- Konfigurationen wie die maximale Kommentarlänge, wenn über die Customization Rule Admin-Seite konfiguriert, gelten auch hier.
- Um Benutzern zu ermöglichen, ihren Kommentartext zu aktualisieren, können Sie einfach `comment` im Anfrage-Body angeben. Wir generieren das resultierende `commentHTML`.
  - Wenn Sie sowohl `comment` als auch `commentHTML` definieren, werden wir das HTML nicht automatisch generieren.
  - Wenn der Benutzer Erwähnungen oder Hashtags in seinem neuen Text hinzufügt, wird es weiterhin wie bei der `POST`-API verarbeitet.
- Beim Aktualisieren von `commenterEmail` bei einem Kommentar ist es am besten, auch `userId` anzugeben. Andernfalls müssen Sie sicherstellen, dass der Benutzer mit dieser E-Mail zu Ihrem Tenant gehört, oder die Anfrage schlägt fehl.


[inline-code-attrs-start title = 'Minimales Comment PATCH cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Comment PATCH Anfrage-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can edit the comment.  **/
    contextUserId?: string
	/** Should we check if the new comment looks like spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment PATCH Antwort-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
