[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Ta ruta omogoča posodobitev ene `Page`. Ustrezni komentarji bodo posodobljeni.

[inline-code-attrs-start title = 'Primer cURL zahteve za posodobitev strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za posodobitev strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora posodobitve strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Vključen ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Vključen ob napaki. **/
    reason?: string
    user?: Page; // Na uspeh vrnemo popolnoma posodobljeno stran.
}
[inline-code-end]

#### Opomba

Nekateri parametri v objektu Page se samodejno posodobijo. To so atributi za števce in naslov. Števcev ni mogoče posodobiti
prek API-ja, saj so izračunane vrednosti. Naslov strani `title` je mogoče nastaviti prek API-ja, vendar bo prepisan, če je gradnik za komentarje uporabljen na
strani z istim `urlId` in drugačnim naslovom strani.