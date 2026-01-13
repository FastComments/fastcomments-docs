FastComments pruža način za konstrukciju pitanja i agregiranje njihovih rezultata. Primer pitanja (u daljem tekstu nazvan `QuestionConfig`)
može biti ocena zvezdicama, klizač, ili NPS pitanje (određeno putem `type`).

Podaci pitanja mogu se agregirati pojedinačno, zajedno, tokom vremena, ukupno, po stranici, i tako dalje.

Framework ima sve mogućnosti potrebne za izgradnju klijentskih widgeta (sa vašim serverom ispred ovog API-ja), administratorskih kontrolnih tabli, i alata za izveštavanje.

Prvo moramo definisati `QuestionConfig`. Struktura je sledeća:

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
    /** SAMO ZA ČITANJE - uvećava se za svaki novi odgovor. **/
    usedCount: number
    /** String datuma koji označava kada je konfiguracija poslednji put korišćena (ostavljen rezultat). **/
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