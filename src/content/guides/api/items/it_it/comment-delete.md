[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Questo endpoint API consente di eliminare un commento.

Note:

- Questa API può aggiornare il comment widget "live" se desiderato (questo aumenta `creditsCost` da `1` a `2`).
- Questa API eliminerà tutti i commenti figli.
- Se il commento target è bloccato (`isLocked: true`), la richiesta viene rifiutata con `code: 'locked'`. Sbloccare prima il commento, poi eliminarlo.

[inline-code-attrs-start title = 'Esempio cURL DELETE Commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struttura della richiesta DELETE Commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** L'utente che esegue l'aggiornamento. Se desiderato può essere usato per verificare che possa eliminare il commento.  **/
    contextUserId?: string
	/** Se il commento dovrebbe essere eliminato "live" per gli utenti che visualizzano istanze del widget dei commenti con lo stesso urlId. NOTA: Raddoppia il costo in crediti da 1 a 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta DELETE Commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]

---