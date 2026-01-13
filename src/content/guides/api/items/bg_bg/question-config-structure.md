FastComments предоставя начин за конструиране на въпроси и агрегиране на техните резултати. Пример за въпрос (наричан по-нататък `QuestionConfig`)
може да бъде звездна оценка, плъзгач или NPS въпрос (определен чрез `type`).

Данните от въпросите могат да бъдат агрегирани индивидуално, заедно, във времето, общо, по страница и така нататък.

Рамката има всички възможности, необходими за изграждане на уиджети от клиентска страна (с вашия сървър пред този API), административни табла и инструменти за отчетност.

Първо трябва да дефинираме `QuestionConfig`. Структурата е следната:

[inline-code-attrs-start title = 'Структура на QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** READONLY - incremented for each new response. **/
    usedCount: number
    /** A date string for when the configuration was last used (result left). **/
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

