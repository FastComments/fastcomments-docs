FastComments пружа начин за конструисање питања и агрегирање њихових резултата. Пример питања (у даљем тексту названо `QuestionConfig`) може бити оцена у звездицама, клизач или NPS питање (одређено помоћу `type`).

Подаци о питањима могу се агрегирати појединачно, заједно, током времена, укупно, по страници итд.

Фрејмворк има све могућности потребне за изградњу клијентских видгета (са вашим сервером испред овог API), административних контролних панела и алата за извјештавање.

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
    /** САМО ЗА ЧИТАЊЕ - увећава се за сваки нови одговор. **/
    usedCount: number
    /** Низ датума који означава када је конфигурација последњи пут коришћена (остављен резултат). **/
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