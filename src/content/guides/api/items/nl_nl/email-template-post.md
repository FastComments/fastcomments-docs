[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Dit API-endpoint biedt de mogelijkheid om e-mailsjablonen te maken.

Opmerkingen:

- Je kunt niet meerdere sjablonen hebben met dezelfde `emailTemplateId` voor hetzelfde domein.
- Maar je kunt wel een wildcard-sjabloon hebben (`domain` = `*`) en een domeinspecifiek sjabloon voor dezelfde `emailTemplateId`.
- Het opgeven van `domain` is alleen relevant als je meerdere domeinen hebt, of specifieke sjablonen wilt gebruiken voor testen (`domain` ingesteld op `localhost` etc).
- Als je `domain` opgeeft, moet dit overeenkomen met een `DomainConfig`. Bij een fout wordt een lijst met geldige domeinen verstrekt.
- De sjabloonsyntaxis is EJS en wordt gerenderd met een timeout van 500ms. P99 voor renderen is <5ms, dus als je de 500ms bereikt is er iets mis.
- **Je sjabloon moet renderen met de opgegeven `testData`** om op te slaan. Renderfouten worden samengevoegd en gerapporteerd op het dashboard (binnenkort via de API beschikbaar). 

De minimale gegevens die nodig zijn om een sjabloon toe te voegen, zijn als volgt:

[inline-code-attrs-start title = 'Minimaal EmailTemplate POST cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Je wilt mogelijk per site sjablonen hebben, in dat geval definieer je `domain`:

[inline-code-attrs-start title = 'EmailTemplate POST cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'EmailTemplate POST Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST Antwoordstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---