Da shranite rezultate za vprašanja, ustvarite `QuestionResult`. Nato lahko združite rezultate vprašanj in jih povežete s komentarji za namene poročanja.

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