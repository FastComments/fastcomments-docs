为了保存问题的结果，您需要创建一个 `QuestionResult`。您随后可以对问题结果进行汇总，并且
将它们与评论关联以用于报告目的。

[inline-code-attrs-start title = 'QuestionResult 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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