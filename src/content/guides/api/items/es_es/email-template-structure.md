Un objeto `EmailTemplate` representa la configuración para una plantilla de email personalizada, para un inquilino.

El sistema seleccionará la plantilla de email a usar mediante:

- Su identificador de tipo, lo llamamos `emailTemplateId`. Estos son constantes.
- El `domain`. Primero intentaremos encontrar una plantilla para el dominio al que está vinculado el objeto relacionado (como un `Comment`), y si no se encuentra una coincidencia entonces intentaremos encontrar una plantilla donde domain sea null o `*`.

La estructura del objeto `EmailTemplate` es la siguiente:

[inline-code-attrs-start title = 'Estructura de Plantilla de Email'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Notas

- Puede obtener los valores válidos de `emailTemplateId` desde el endpoint `/definitions`.
- El endpoint `/definitions` también incluye las traducciones predeterminadas y datos de prueba.
- Las plantillas fallarán al guardar con estructura o datos de prueba inválidos.
