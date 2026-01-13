Sorular için sonuçları kaydetmek amacıyla bir `QuestionResult` oluşturursunuz. Ardından soru sonuçlarını toplu hâlde işleyebilir ve raporlama amaçları için yorumlara bağlayabilirsiniz.

[inline-code-attrs-start title = 'QuestionResult Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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