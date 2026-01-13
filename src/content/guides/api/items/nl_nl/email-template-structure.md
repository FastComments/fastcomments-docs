Een `EmailTemplate`-object vertegenwoordigt de configuratie voor een aangepast e-mailsjabloon voor een tenant.

Het systeem selecteert het e-mailsjabloon dat gebruikt wordt via:

- Het type-identifier, dit noemen we `emailTemplateId`. Dit zijn constanten.
- De `domain`. We proberen eerst een sjabloon te vinden voor de domain waaraan het gerelateerde object (zoals een `Comment`) is gekoppeld, en als er geen overeenkomst wordt gevonden zullen we proberen een sjabloon te vinden waarbij domain null of `*` is.

De structuur van het `EmailTemplate`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van het EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** ALLEEN LEZEN **/
    createdAt: string
    /** ALLEEN LEZEN **/
    updatedAt: string
    /** ALLEEN LEZEN **/
    updatedByUserId: string
    /** De domain waaraan het sjabloon gekoppeld moet worden. **/
    domain?: string | '*' | null
    /** De inhoud van het e-mailsjabloon in EJS-syntaxis. **/
    ejs: string
    /** Een map van overschreven vertaalingssleutels naar waarden, voor elke ondersteunde locale. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Een object dat de rendercontext van het sjabloon vertegenwoordigt. **/
    testData: object
}
[inline-code-end]

### Opmerkingen

- Je kunt de geldige `emailTemplateId`-waarden verkrijgen via het `/definitions`-endpoint.
- Het `/definitions`-endpoint bevat ook de standaardvertalingen en testgegevens.
- Sjablonen slaan niet op als de structuur of testgegevens ongeldig zijn.