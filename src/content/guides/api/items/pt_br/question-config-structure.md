O FastComments fornece uma forma de construir perguntas e agregar seus resultados. Um exemplo de pergunta (doravante chamada `QuestionConfig`)
pode ser uma avaliação em estrelas, um slider ou uma pergunta NPS (determinada via `type`).

Os dados das perguntas podem ser agregados individualmente, em conjunto, ao longo do tempo, de forma geral, por página e assim por diante.

O framework possui todas as capacidades necessárias para construir widgets no lado do cliente (com seu servidor na frente desta API), painéis administrativos e ferramentas de relatórios.

Primeiro, temos que definir uma `QuestionConfig`. A estrutura é a seguinte:

[inline-code-attrs-start title = 'Estrutura de QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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