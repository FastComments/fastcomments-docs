FastComments fournit un moyen de construire des questions et d'agréger leurs résultats. Un exemple de question (ci-après appelée `QuestionConfig`)
pourrait être une évaluation par étoiles, un curseur, ou une question NPS (déterminé via `type`).

Les données de question peuvent être agrégées individuellement, ensemble, au fil du temps, globalement, par page, et ainsi de suite.

Le framework a toutes les capacités nécessaires pour construire des widgets côté client (avec votre serveur devant cette API), des tableaux de bord d'administration, et des outils de rapports.

D'abord, nous devons définir un `QuestionConfig`. La structure est la suivante:

[inline-code-attrs-start title = 'Structure de QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

