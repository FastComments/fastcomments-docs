FastComments fornisce un modo per creare domande e aggregarne i risultati. Un esempio di domanda (d'ora in poi chiamata `QuestionConfig`) potrebbe essere una valutazione a stelle, uno slider, o una domanda NPS (determinata tramite `type`).

I dati delle domande possono essere aggregati individualmente, insieme, nel tempo, complessivamente, per pagina, e così via.

Il framework dispone di tutte le funzionalità necessarie per costruire widget client-side (con il tuo server davanti a questa API), pannelli di amministrazione e strumenti di reporting.

Per prima cosa, dobbiamo definire un `QuestionConfig`. La struttura è la seguente:

[inline-code-attrs-start title = 'Struttura di QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** SOLA LETTURA - incrementato per ogni nuova risposta. **/
    usedCount: number
    /** Una stringa di data che indica quando la configurazione è stata usata l'ultima volta (quando è stato lasciato un risultato). **/
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