FastComments bietet eine Möglichkeit, Fragen zu konstruieren und ihre Ergebnisse zu aggregieren. Ein Beispiel für eine Frage (im Folgenden `QuestionConfig` genannt)
könnte eine Sternebewertung, ein Schieberegler oder eine NPS-Frage sein (bestimmt durch `type`).

Fragedaten können einzeln, zusammen, über Zeit, insgesamt, nach Seite usw. aggregiert werden.

Das Framework hat alle Fähigkeiten, die benötigt werden, um clientseitige Widgets (mit Ihrem Server vor dieser API), Admin-Dashboards und Reporting-Tools zu erstellen.

Zuerst müssen wir eine `QuestionConfig` definieren. Die Struktur ist wie folgt:

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
