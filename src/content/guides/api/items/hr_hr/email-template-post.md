[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućuje kreiranje predložaka e-pošte.

Bilješke:

- Ne možete imati više predložaka s istim `emailTemplateId` za istu domenu.
- Međutim, možete imati univerzalni predložak (`domain` = `*`) i predložak specifičan za domenu za isti `emailTemplateId`.
- Navođenje `domain` ima smisla samo ako imate različite domene ili želite koristiti specifične predloške za testiranje (`domain` postavljen na `localhost` itd.).
- Ako navedete `domain`, mora odgovarati `DomainConfig`. U slučaju greške daje se popis valjanih domena.
- Sintaksa predloška je EJS i renderira se s timeoutom od 500 ms. P99 za renderiranje je <5 ms, tako da ako dosegnete 500 ms nešto nije u redu.
- **Vaš predložak mora renderirati s danim `testData`** da bi se spremio. Greške pri renderiranju se agregiraju i prikazuju u nadzornoj ploči (uskoro dostupno putem API-ja). 

Minimalni podaci potrebni za dodavanje predloška su sljedeći:

[inline-code-attrs-start title = 'Minimalni EmailTemplate POST cURL primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

Možda ćete htjeti imati predloške po web-mjestu, u kojem slučaju definirate `domain`:

[inline-code-attrs-start title = 'EmailTemplate POST cURL primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura EmailTemplate POST zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura EmailTemplate POST odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Kreirani predložak. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---