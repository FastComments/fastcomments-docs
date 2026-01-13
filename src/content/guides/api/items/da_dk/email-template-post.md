[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Denne API-endpoint giver mulighed for at oprette e-mail-skabeloner.

Bemærkninger:

- Du kan ikke have flere skabeloner med det samme `emailTemplateId` med det samme domæne.
- Men du kan have en wildcard-skabelon (`domain` = `*` og en domænespecifik skabelon for det samme `emailTemplateId`).
- Angivelse af `domain` er kun relevant, hvis du har forskellige domæner, eller ønsker at bruge specifikke skabeloner til test (`domain` sat til `localhost` osv.).
- Hvis du angiver `domain`, skal det matche en `DomainConfig`. Ved fejl gives en liste over gyldige domæner.
- Skabelonsyntaksen er EJS og renderes med en timeout på 500ms. P99 for rendering er <5ms, så hvis du rammer 500ms, er der noget galt.
- **Din skabelon skal kunne renderes med dine givne `testData`** for at gemme. Renderingsfejl aggregeres og rapporteres i dashboardet (snart tilgængeligt via API).

De minimale data, der kræves for at tilføje en skabelon, er som følger:

[inline-code-attrs-start title = 'Minimum EmailTemplate POST cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Du vil måske have skabeloner per-site, i hvilket tilfælde du definerer `domain`:

[inline-code-attrs-start title = 'EmailTemplate POST cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'EmailTemplate POST Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The created template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
