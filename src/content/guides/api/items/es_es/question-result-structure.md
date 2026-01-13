Para guardar resultados de preguntas, crea un `QuestionResult`. Luego puede agregar resultados de preguntas, y también
vincularlos a comentarios para propósitos de informes.

[inline-code-attrs-start title = 'Estructura de QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultMeta {
    name: string
    values: string[]
}

interface QuestionResult {
    id: string
    tenantId: string
    urlId: string
    anonUserId?: string
    userId?: string
    createdAt?: string
    value: number
    commentId?: string
    questionId: string
    meta?: QuestionResultMeta[]
}
[inline-code-end]
