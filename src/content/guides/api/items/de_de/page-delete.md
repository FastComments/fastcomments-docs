[api-resource-header-start name = 'Page'; route = 'DELETE /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Diese Route ermöglicht das Entfernen einer einzelnen Seite nach ID.

Beachten Sie, dass die Interaktion mit dem Kommentar-Widget für eine Seite mit derselben `urlId` die `Page` einfach nahtlos neu erstellt.

[inline-code-attrs-start title = 'Seiten Entfernen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pages/some-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Seiten Entfernen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Seiten Entfernen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'page-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
