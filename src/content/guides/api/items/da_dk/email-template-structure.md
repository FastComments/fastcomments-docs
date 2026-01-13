Et `EmailTemplate`-objekt repræsenterer konfiguration for en tilpasset e-mail-skabelon, for en tenant.

Systemet vil vælge e-mail-skabelonen, der skal bruges via:

- Dens typeidentifikator, vi kalder denne `emailTemplateId`. Disse er konstanter.
- `domain`. Vi vil først forsøge at finde en skabelon for domænet, som det relaterede objekt (såsom en `Comment`) er knyttet til, og hvis et match ikke findes, vil vi forsøge at finde en skabelon, hvor domain er null eller `*`.

Strukturen for `EmailTemplate`-objektet er som følger:

[inline-code-attrs-start title = 'Email Template Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** READONLY **/
    createdAt: string
    /** READONLY **/
    updatedAt: string
    /** READONLY **/
    updatedByUserId: string
    /** The domain the template should be associated with. **/
    domain?: string | '*' | null
    /** The email template content in EJS syntax. **/
    ejs: string
    /** A map of overridden translation keys to values, for each supported locale. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** An object that represents the render context of the template. **/
    testData: object
}
[inline-code-end]

### Bemærkninger

- Du kan få de gyldige `emailTemplateId`-værdier fra `/definitions`-endpointet.
- `/definitions`-endpointet inkluderer også standardoversættelserne og testdata.
- Skabeloner vil fejle ved gemning med ugyldig struktur eller testdata.
