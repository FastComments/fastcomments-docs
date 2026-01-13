FastComments giver en måde at konstruere spørgsmål og aggregere deres resultater. Et eksempel på et spørgsmål (herefter kaldet `QuestionConfig`)
kunne være en stjernebedømmelse, en skyder eller et NPS-spørgsmål (bestemt via `type`).

Spørgsmålsdata kan aggregeres individuelt, sammen, over tid, samlet, efter side og så videre.

Frameworket har alle de nødvendige funktioner til at bygge klientside-widgets (med din server foran dette API), admin-dashboards og rapporteringsværktøjer.

Først skal vi definere en `QuestionConfig`. Strukturen er som følger:

[inline-code-attrs-start title = 'QuestionConfig Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
