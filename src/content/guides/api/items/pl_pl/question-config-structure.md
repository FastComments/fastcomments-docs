FastComments zapewnia sposób tworzenia pytań i agregowania ich wyników. Przykładem pytania (dalej nazywanego `QuestionConfig`) może być ocena w gwiazdkach, suwak lub pytanie NPS (określane przez `type`).

Dane z pytań można agregować indywidualnie, łącznie, w czasie, ogólnie, według strony i tak dalej.

Framework posiada wszystkie niezbędne możliwości do budowy widgetów po stronie klienta (z Twoim serwerem przed tym API), paneli administracyjnych i narzędzi raportujących.

Najpierw musimy zdefiniować `QuestionConfig`. Struktura jest następująca:

[inline-code-attrs-start title = 'Struktura QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** TYLKO DO ODCZYTU - zwiększane przy każdej nowej odpowiedzi. **/
    usedCount: number
    /** Ciąg znaków z datą określający, kiedy konfiguracja była ostatnio użyta (gdy pozostawiono wynik). **/
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