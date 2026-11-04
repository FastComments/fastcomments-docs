[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Ten punkt końcowy API umożliwia tworzenie szablonów e-mail.

Uwagi:

- Nie możesz mieć wielu szablonów z tym samym `emailTemplateId` dla tej samej domeny.
- Ale możesz mieć szablon wieloznaczny (`domain` = `*` oraz szablon specyficzny dla domeny dla tego samego `emailTemplateId`).
- Określanie `domain` ma znaczenie tylko wtedy, gdy masz różne domeny lub chcesz używać specyficznych szablonów do testów (`domain` ustawione na `localhost` itd.).
- Jeśli określisz `domain`, musi ono pasować do `DomainConfig`. W przypadku błędu zwracana jest lista prawidłowych domen.
- Składnia szablonu to EJS i jest renderowana z limitem czasu 500 ms. P99 dla renderowania wynosi <5 ms, więc jeśli osiągniesz 500 ms, coś jest nie tak.
- **Twój szablon musi się wyrenderować z podanym `testData`**, aby mógł zostać zapisany. Błędy renderowania są agregowane i raportowane w panelu (wkrótce dostępne przez API).

Minimalne dane wymagane do dodania szablonu są następujące:

[inline-code-attrs-start title = 'Minimalny przykład cURL POST EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Możesz chcieć mieć szablony per strona — w takim przypadku definiujesz `domain`:

[inline-code-attrs-start title = 'Przykład cURL POST EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura żądania POST EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi POST EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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