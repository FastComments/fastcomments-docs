[api-resource-header-start name = 'HashTag'; route = 'DELETE /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

Diese Route ermöglicht das Entfernen eines `HashTag`-Benutzers anhand des angegebenen Tags.

Beachten Sie, dass Hashtags, sofern die automatische `HashTag`-Erstellung nicht deaktiviert ist, von einem Benutzer neu erstellt werden können, indem er beim Kommentieren den Hashtag angibt.

[inline-code-attrs-start title = 'HashTag Entfernen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag Entfernen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag Entfernen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
