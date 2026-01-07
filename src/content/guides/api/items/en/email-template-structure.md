An `EmailTemplate` object represents configuration for a custom email template, for a tenant.

The system will select the email template to use via:

- Its type identifier, we call this `emailTemplateId`. These are constants.
- The `domain`. We will first try to find a template for the domain that the related object (like a `Comment`) is tied to, and if a match is not found then we will try to find a template where domain is null or `*`.

The structure for the `EmailTemplate` object is as follows:

[inline-code-attrs-start title = 'Email Template Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Notes

- You can get the valid `emailTemplateId` values from the `/definitions` endpoint.
- The `/definitions` endpoint also includes the default translations and test data.
- Templates will fail to save with invalid structure or test data.
