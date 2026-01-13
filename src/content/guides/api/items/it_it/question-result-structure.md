Per salvare i risultati delle domande, crei un `QuestionResult`. Puoi quindi aggregare i risultati delle domande e anche collegarli ai commenti per scopi di reportistica.

[inline-code-attrs-start title = 'Struttura di QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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