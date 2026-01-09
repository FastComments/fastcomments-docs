FastComments pruža način za konstruiranje pitanja i agregiranje njihovih rezultata. Primjer pitanja (u daljnjem tekstu nazvan `QuestionConfig`) može biti ocjena zvjezdicama, klizač, ili NPS pitanje (određeno putem `type`).

Podaci o pitanjima mogu se agregirati pojedinačno, zajedno, tijekom vremena, ukupno, po stranici i slično.

Okvir ima sve mogućnosti potrebne za izgradnju klijentskih widgeta (s vašim serverom ispred ovog API-ja), administratorskih nadzornih ploča i alata za izvještavanje.

Prvo moramo definirati a `QuestionConfig`. Struktura je sljedeća:

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
    /** SAMO ZA ČITANJE - povećava se za svaki novi odgovor. **/
    usedCount: number
    /** Niz datuma koji označava kada je konfiguracija zadnji put korištena (ostavljen je rezultat). **/
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