FastComments fornece uma maneira de construir perguntas e agregar seus resultados. Um exemplo de pergunta (doravante chamado `QuestionConfig`) pode ser uma avaliação por estrelas, um controle deslizante ou uma pergunta NPS (determinada via `type`).

Os dados das perguntas podem ser agregados individualmente, em conjunto, ao longo do tempo, no geral, por página, e assim por diante.

O framework possui todas as capacidades necessárias para construir widgets no lado do cliente (com seu servidor na frente desta API), painéis de administração e ferramentas de relatório.

First, we have to define a `QuestionConfig`. The structure is as follows:

[inline-code-attrs-start title = 'Estrutura do QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** SOMENTE LEITURA - incrementado a cada nova resposta. **/
    usedCount: number
    /** Uma string de data para quando a configuração foi usada pela última vez (resposta enviada). **/
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