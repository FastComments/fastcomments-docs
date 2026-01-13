Да бисте сачували резултате за питања, креирате `QuestionResult`. Затим можете агрегирати резултате питања, и такође их повезати са коментарима у сврху извештавања.

[inline-code-attrs-start title = 'Struktura QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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