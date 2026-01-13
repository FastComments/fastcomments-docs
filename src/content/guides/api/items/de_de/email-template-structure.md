Ein `EmailTemplate`-Objekt repräsentiert die Konfiguration für eine benutzerdefinierte E-Mail-Vorlage für einen Tenant.

Das System wählt die zu verwendende E-Mail-Vorlage über:

- Ihren Typ-Identifikator, wir nennen dies `emailTemplateId`. Dies sind Konstanten.
- Die `domain`. Wir versuchen zuerst, eine Vorlage für die Domain zu finden, mit der das zugehörige Objekt (wie ein `Comment`) verbunden ist, und wenn keine Übereinstimmung gefunden wird, versuchen wir, eine Vorlage zu finden, bei der domain null oder `*` ist.

Die Struktur des `EmailTemplate`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'E-Mail-Vorlagen Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Hinweise

- Sie können die gültigen `emailTemplateId`-Werte vom `/definitions`-Endpunkt abrufen.
- Der `/definitions`-Endpunkt enthält auch die Standardübersetzungen und Testdaten.
- Vorlagen können nicht gespeichert werden, wenn sie eine ungültige Struktur oder Testdaten haben.
