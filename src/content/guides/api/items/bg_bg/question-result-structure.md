За да запазите резултати от въпроси, създавате `QuestionResult`. След това можете да агрегирате резултатите от въпросите и също
да ги свържете с коментари за целите на отчетността.

[inline-code-attrs-start title = 'Структура на QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
