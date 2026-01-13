FastComments biedt een manier om vragen te maken en de resultaten ervan te aggregeren. Een voorbeeld van een vraag (hierna aangeduid als `QuestionConfig`) kan een sterrenbeoordeling, een schuifregelaar of een NPS-vraag zijn (bepaald via `type`).

Vraaggegevens kunnen individueel, gezamenlijk, over de tijd, in totaal, per pagina, enzovoort worden geaggregeerd.

Het framework bevat alle mogelijkheden die nodig zijn om client-side widgets (met uw server voor deze API), beheerdersdashboards en rapportagetools te bouwen.

Eerst moeten we een `QuestionConfig` definiëren. De structuur is als volgt:

[inline-code-attrs-start title = 'Structuur van QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** ALLEEN LEZEN - wordt verhoogd voor elk nieuw antwoord. **/
    usedCount: number
    /** Een datumstring die aangeeft wanneer de configuratie voor het laatst is gebruikt (wanneer er een resultaat is achtergelaten). **/
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