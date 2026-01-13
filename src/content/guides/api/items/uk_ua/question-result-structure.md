Щоб зберегти результати для питань, ви створюєте `QuestionResult`. Потім ви можете агрегувати результати питань, а також
зв'язувати їх із коментарями для цілей звітності.

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