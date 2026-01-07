[api-resource-header-start name = 'Aggregate'; route = 'POST /api/v1/aggregate'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at aggregere data for et vilkårligt sæt urlIds. Du kan ikke lave flere end `1000` urlIds pr. anmodning.

Her er definitionen af `AggregatePost`:

[inline-code-attrs-start title = 'AggregatePost Definitionsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AggregatePostBody {
    urlIds: string[]
}
[inline-code-end]

Resultatet er en map af `urlId` til en aggregeret objekt med `count`, som er antallet af kommentarer, og `commentCountsByDate`, som er en map af dato til antal kommentarer.

[inline-code-attrs-start title = 'AggregatePost cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "urlIds": ["some-page-id-or-url"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'AggregatePost Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AggregatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'AggregatePost Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AggregatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'too-many-url-ids'
    /** Included on failure. **/
    reason?: string
    data: Record<string, { count: number; commentCountsByDate: Record<string, number> }>
}
[inline-code-end]
