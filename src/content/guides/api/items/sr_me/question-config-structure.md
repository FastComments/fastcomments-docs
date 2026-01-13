FastComments pruža način za konstruisanje pitanja i agregiranje njihovih rezultata. Primjer pitanja (u daljem tekstu nazvan `QuestionConfig`) može biti ocjena sa zvjezdicama, klizač, ili NPS pitanje (određeno putem `type`).

Podaci o pitanjima mogu se agregirati pojedinačno, zajedno, tokom vremena, ukupno, po stranici, i tako dalje.

Okvir ima sve mogućnosti potrebne za izgradnju klijentskih vidžeta (sa vašim serverom ispred ovog API-ja), administratorskih kontrolnih tabli i alata za izvještavanje.

Prvo, moramo definisati `QuestionConfig`. Struktura je sljedeća:

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
    /** String datuma koji označava kada je konfiguracija posljednji put korišćena (ostavljen rezultat). **/
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