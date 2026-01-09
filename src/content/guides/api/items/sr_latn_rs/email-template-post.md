[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint pruža mogućnost kreiranja email šablona.

Napomene:

- Ne možete imati više šablona sa istim `emailTemplateId` za istu domenu.
- Međutim, možete imati wildcard šablon (`domain` = `*` and a domain specific template for the same `emailTemplateId`).
- Navođenje `domain` je relevantno samo ako imate različite domene, ili želite koristiti specifične šablone za testiranje (`domain` set to `localhost` etc).
- Ako navedete `domain` it must match a `DomainConfig`. On error a list of valid domains is provided.
- Sintaksa šablona je EJS i renderuje se sa timeout-om od 500ms. P99 za renderovanje je <5ms, tako da ako dosegnete 500ms nešto nije u redu.
- **Vaš šablon mora biti renderovan sa zadatim `testData`** da bi se sačuvao. Greške pri renderovanju se agregiraju i prijavljuju na dashboard (uskoro dostupno preko API-ja). 

Minimalni podaci potrebni za dodavanje šablona su sledeći:

[inline-code-attrs-start title = 'Minimalni EmailTemplate POST cURL primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Možda ćete želeti da imate šablone po sajtu, u kom slučaju definišete `domain`:

[inline-code-attrs-start title = 'EmailTemplate POST cURL primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura EmailTemplate POST zahteva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Kreirani šablon. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]