Објекат `EmailTemplate` представља конфигурацију за прилагођени имејл шаблон, за tenant.

Систем ће одабрати имејл шаблон који ће се користити путем:

- Његовог идентификатора типа, називамо га `emailTemplateId`. Ово су константе.
- `domain`. Прво ћемо покушати пронаћи шаблон за домен уз који је повезан релевантни објекат (попут `Comment`), а ако се поклапање не пронађе онда ћемо покушати пронаћи шаблон где је domain null или `*`.

Структура за објекат `EmailTemplate` је следећа:

[inline-code-attrs-start title = 'Структура имејл шаблона'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** САМО ЗА ЧИТАЊЕ **/
    createdAt: string
    /** САМО ЗА ЧИТАЊЕ **/
    updatedAt: string
    /** САМО ЗА ЧИТАЊЕ **/
    updatedByUserId: string
    /** Домен са којим шаблон треба бити повезан. **/
    domain?: string | '*' | null
    /** Садржај имејл шаблона у EJS синтакси. **/
    ejs: string
    /** Мапа преписаних кључева превода у вредности за сваки подржани локал. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Објекат који представља контекст рендеровања шаблона. **/
    testData: object
}
[inline-code-end]

### Напомене

- You can get the valid `emailTemplateId` values from the `/definitions` endpoint.
- The `/definitions` endpoint also includes the default translations and test data.
- Шаблони неће бити сачувани ако структура или тестни подаци нису валидни.