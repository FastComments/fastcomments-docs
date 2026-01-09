Чтобы сохранить результаты для вопросов, вы создаёте `QuestionResult`. Затем вы можете агрегировать результаты вопросов, и также
привязывать их к комментариям для целей отчётности.

[inline-code-attrs-start title = 'Структура QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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