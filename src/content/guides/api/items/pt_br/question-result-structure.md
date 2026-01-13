Para salvar resultados de perguntas, você cria um `QuestionResult`. Você pode então agregar resultados de perguntas, e também vinculá-los a comentários para fins de relatório.

[inline-code-attrs-start title = 'Estrutura de QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---