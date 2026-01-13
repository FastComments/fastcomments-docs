FastComments proporciona una forma de construir preguntas y agregar sus resultados. Un ejemplo de una pregunta (en adelante llamada `QuestionConfig`)
podría ser una calificación de estrellas, un deslizador, o una pregunta NPS (determinado vía `type`).

Los datos de preguntas pueden agregarse individualmente, juntos, a lo largo del tiempo, en general, por página, y así sucesivamente.

El framework tiene todas las capacidades necesarias para construir widgets del lado del cliente (con su servidor frente a esta API), dashboards de administración, y herramientas de informes.

Primero, tenemos que definir un `QuestionConfig`. La estructura es la siguiente:

[inline-code-attrs-start title = 'Estructura de QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

