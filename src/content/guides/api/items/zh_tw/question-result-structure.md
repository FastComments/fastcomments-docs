為了儲存問題的結果，您需要建立一個 `QuestionResult`。接著您可以彙總問題結果，並且將它們
與評論關聯以便進行報告。

[inline-code-attrs-start title = 'QuestionResult 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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