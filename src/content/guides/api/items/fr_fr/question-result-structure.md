Afin de sauvegarder les résultats des questions, vous créez un `QuestionResult`. Vous pouvez ensuite agréger les résultats de questions, et aussi
les lier aux commentaires à des fins de rapports.

[inline-code-attrs-start title = 'Structure de QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
