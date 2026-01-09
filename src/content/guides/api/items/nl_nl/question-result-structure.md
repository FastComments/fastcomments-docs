---
Om resultaten voor vragen op te slaan, maakt u een `QuestionResult`. U kunt vervolgens vraagresultaten aggregeren, en ook
ze aan opmerkingen koppelen voor rapportagedoeleinden.

[inline-code-attrs-start title = 'Structuur van QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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