An `EmailTemplate` object represents configuration for a custom email template, for a tenant.

The system will select the email template to use via:

- Its type identifier, we call this `emailTemplateId`. These are constants.
- The `domain`. We will first try to find a template for the domain that the related object (like a `Comment`) is tied to, and if a match is not found then we will try to find a template where domain is null or `*`.

The structure for the `EmailTemplate` object is as follows:

[inline-code-attrs-start title = 'Struktura predloška e-pošte'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** SAMO ZA ČITANJE **/
    createdAt: string
    /** SAMO ZA ČITANJE **/
    updatedAt: string
    /** SAMO ZA ČITANJE **/
    updatedByUserId: string
    /** Domena s kojom predložak treba biti povezan. **/
    domain?: string | '*' | null
    /** Sadržaj email predloška u EJS sintaksi. **/
    ejs: string
    /** Mapa zamijenjenih prevodnih ključeva na vrijednosti, za svaki podržani lokal. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Objekt koji predstavlja kontekst renderiranja predloška. **/
    testData: object
}
[inline-code-end]

### Notes

- You can get the valid `emailTemplateId` values from the `/definitions` endpoint.
- The `/definitions` endpoint also includes the default translations and test data.
- Templates will fail to save with invalid structure or test data.

---