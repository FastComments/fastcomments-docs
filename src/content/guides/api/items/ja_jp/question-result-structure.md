質問の結果を保存するには、`QuestionResult` を作成します。次に、質問の結果を集計したり、また
レポート目的でコメントに紐付けたりできます。

[inline-code-attrs-start title = 'QuestionResult の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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