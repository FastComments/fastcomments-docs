FastComments nudi način za sestavljanje vprašanj in združevanje njihovih rezultatov. Primer vprašanja (v nadaljevanju imenovan `QuestionConfig`) je lahko ocena z zvezdicami, drsnik ali NPS vprašanje (določeno z `type`).

Podatke o vprašanjih je mogoče združevati posamično, skupaj, skozi čas, na splošno, po strani itd.

Okvir ima vse zmogljivosti, potrebne za izdelavo odjemalskih vtičnikov (z vašim strežnikom pred tem API-jem), nadzornih plošč za skrbnike in orodij za poročanje.

Najprej moramo definirati `QuestionConfig`. Struktura je naslednja:

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
    /** SAMO ZA BRANJE - poveča se za vsak nov odgovor. **/
    usedCount: number
    /** Niz datuma, kdaj je bila konfiguracija nazadnje uporabljena (ko je bil oddan rezultat). **/
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