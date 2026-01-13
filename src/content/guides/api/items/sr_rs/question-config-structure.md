FastComments пружа начин за конструкцију питања и агрегирање њихових резултата. Пример питања (даље названо `QuestionConfig`) може бити оцена у звездицама, клизач, или NPS питање (одређено преко `type`).

Подаци о питањима могу се агрегирати појединачно, заједно, током времена, укупно, по страници и слично.

Овај фрејмворк има све могућности потребне за израду клијентских виџета (са вашим сервером испред овог API-ја), администраторских контролних панела и алата за извештавање.

Прво морамо дефинисати `QuestionConfig`. Структура је следећа:

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
    /** САМО ЗА ЧИТАЊЕ - повећава се за сваки нови одговор. **/
    usedCount: number
    /** Стринг који представља датум када је конфигурација последњи пут коришћена (остављен резултат). **/
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