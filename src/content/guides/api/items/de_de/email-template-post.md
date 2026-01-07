[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, E-Mail-Vorlagen zu erstellen.

Hinweise:

- Sie können nicht mehrere Vorlagen mit derselben `emailTemplateId` und derselben Domain haben.
- Sie können jedoch eine Wildcard-Vorlage (`domain` = `*`) und eine domainspezifische Vorlage für dieselbe `emailTemplateId` haben.
- Die Angabe von `domain` ist nur relevant, wenn Sie verschiedene Domains haben oder spezifische Vorlagen zum Testen verwenden möchten (`domain` auf `localhost` usw. gesetzt).
- Wenn Sie `domain` angeben, muss es mit einer `DomainConfig` übereinstimmen. Bei einem Fehler wird eine Liste gültiger Domains bereitgestellt.
- Die Vorlagensyntax ist EJS und wird mit einem Timeout von 500ms gerendert. P99 für das Rendern liegt bei <5ms, wenn Sie also 500ms erreichen, stimmt etwas nicht.
- **Ihre Vorlage muss mit Ihren angegebenen `testData` gerendert werden**, um zu speichern. Renderfehler werden aggregiert und im Dashboard gemeldet (bald über API verfügbar).

Die Mindestdaten, die zum Hinzufügen einer Vorlage erforderlich sind, sind wie folgt:

[inline-code-attrs-start title = 'Minimales EmailTemplate POST cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Möglicherweise möchten Sie Vorlagen pro Website haben, in diesem Fall definieren Sie `domain`:

[inline-code-attrs-start title = 'EmailTemplate POST cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'EmailTemplate POST Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
