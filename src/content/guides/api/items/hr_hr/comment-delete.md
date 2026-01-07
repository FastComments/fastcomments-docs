[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ova API krajnja točka pruža mogućnost brisanja komentara.

Napomene:

- Ovaj API može ažurirati widget za komentare "uživo" ako je potrebno (ovo povećava `creditsCost` s `1` na `2`).
- Ovaj API će izbrisati sve podređene komentare.

[inline-code-attrs-start title = 'Primjer cURL za DELETE komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahtjeva za DELETE komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can delete the comment.  **/
    contextUserId?: string
	/** Whether the comment should be deleted "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za DELETE komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
