[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Ta API vmesnik omogoča ustvarjanje predlog e-pošte.

Opombe:

- Ne morete imeti več predlog z istim `emailTemplateId` za isto domeno.
- Vendar pa lahko imate univerzalno (wildcard) predlogo (`domain` = `*`) in domensko specifično predlogo za isti `emailTemplateId`.
- Določitev `domain` je pomembna le, če imate različne domene ali želite uporabiti specifične predloge za testiranje (npr. `domain` nastavljena na `localhost`).
- Če določite `domain`, mora ta ustrezati `DomainConfig`. V primeru napake je na voljo seznam veljavnih domen.
- Sintaksa predloge je EJS in se upodablja z omejitvijo 500ms. P99 za upodabljanje je <5ms, zato, če dosežete 500ms, je nekaj narobe.
- **Vaša predloga se mora upodabljati z danimi `testData`**, da se shrani. Napake pri upodabljanju se združujejo in poročajo na nadzorni plošči (kmalu na voljo tudi prek API).

Najmanj podatkov, potrebnih za dodajanje predloge, je naslednjih:

[inline-code-attrs-start title = 'Minimalni primer cURL POST za EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Morda boste želeli imeti predloge za vsako spletno mesto posebej; v tem primeru določite `domain`:

[inline-code-attrs-start title = 'Primer cURL POST za EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura POST zahteve za EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST odgovora za EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Vključeno ob neuspehu. **/
    reason?: string
    /** Ustvarjena predloga. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---