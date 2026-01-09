FastComments предоставляет способ создания вопросов и агрегации их результатов. Пример вопроса (в дальнейшем называемого `QuestionConfig`) может быть рейтингом в виде звезд, ползунком или вопросом NPS (определяется через `type`).

Данные вопросов можно агрегировать по отдельности, вместе, по времени, в целом, по странице и т.д.

Фреймворк предоставляет все возможности, необходимые для создания клиентских виджетов (с вашим сервером перед этим API), административных панелей и инструментов отчетности.

Сначала нужно определить `QuestionConfig`. Структура выглядит следующим образом:

[inline-code-attrs-start title = 'Структура QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type QuestionConfigType = 'nps' | 'slider' | 'star' | 'thumbs';

interface QuestionConfig {
    id: string
    tenantId: string
    name: string
    question: string
    helpText?: string
    createdAt: string
    createdBy: string
    /** ТОЛЬКО ДЛЯ ЧТЕНИЯ - увеличивается при каждом новом ответе. **/
    usedCount: number
    /** Строка с датой, когда конфигурация была в последний раз использована (оставлен результат). **/
    lastUsed?: string
    type: QuestionConfigType
    numStars?: number
    min?: number
    max?: number
    defaultValue?: number
    labelNegative?: string
    labelPositive?: string
    subQuestionIds?: string[]
    alwaysShowSubQuestions?: boolean
    reportingOrder: number
}
[inline-code-end]


---