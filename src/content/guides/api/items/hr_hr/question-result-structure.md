Da biste spremili rezultate za pitanja, stvorite `QuestionResult`. Zatim možete agregirati rezultate pitanja, i također
povezati ih s komentarima u svrhu izvještavanja.

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