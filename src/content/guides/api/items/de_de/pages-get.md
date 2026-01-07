[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Sie können derzeit nur alle Seiten (oder eine einzelne Seite über `/by-url-id`) abrufen, die mit Ihrem Konto verknüpft sind. Wenn Sie eine feinere Suche wünschen, [kontaktieren Sie uns](https://fastcomments.com/auth/my-account/help).

[inline-code-attrs-start title = 'Seiten cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Seiten Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Seiten Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Hilfreicher Tipp

Die `Comment`-API erfordert eine `urlId`. Sie können zuerst die `Pages`-API aufrufen, um zu sehen, wie die verfügbaren `urlId`-Werte
aussehen.
