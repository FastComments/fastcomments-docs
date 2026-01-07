FastComments provides a way to construct questions and aggregate their results. An example of a question (hereafter called `QuestionConfig`)
could be a star rating, a slider, or an NPS question (determined via `type`).

Question data can be aggregated individually, together, over time, overall, by page, and so on.

The framework has all the capabilities needed to build client-side widgets (with your server in front of this API), admin dashboards, and reporting tools.

First, we have to define a `QuestionConfig`. The structure is as follows:

[inline-code-attrs-start title = 'QuestionConfig Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

