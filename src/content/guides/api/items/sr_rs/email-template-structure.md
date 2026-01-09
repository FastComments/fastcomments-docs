Објекат `EmailTemplate` представља конфигурацију за прилагођени шаблон е-поште за тенанта.

Систем ће изабрати шаблон е-поште који треба да се користи на основу:

- Његов идентификатор типа, то називамо `emailTemplateId`. Ово су константе.
- `domain`. Прво ћемо покушати да пронађемо шаблон за домен са којим је повезан релевантни објекат (као што је `Comment`), а ако се поклапање не пронађе онда ћемо покушати да нађемо шаблон где је domain null или `*`.

Структура објекта `EmailTemplate` је следећа:

[inline-code-attrs-start title = 'Структура шаблона е-поште'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Домен са којим шаблон треба да буде повезан. **/
    domain?: string | '*' | null
    /** Садржај шаблона е-поште у EJS синтакси. **/
    ejs: string
    /** Мапа замењених кључева превода у вредности, за сваку подржану локалу. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Објекат који представља контекст рендеровања шаблона. **/
    testData: object
}
[inline-code-end]

### Напомене

- Можете добити важеће вредности `emailTemplateId` са ендпоинта `/definitions`.
- Ендпоинт `/definitions` такође укључује подразумеване преводе и тест податке.
- Шаблони неће бити сачувани ако имају неважећу структуру или неважеће тест податке.

---