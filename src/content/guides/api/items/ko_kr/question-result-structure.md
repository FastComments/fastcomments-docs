질문에 대한 결과를 저장하려면 `QuestionResult`를 생성합니다. 그런 다음 질문 결과를 집계할 수 있고, 또한
보고 목적을 위해 댓글에 연결할 수도 있습니다.

[inline-code-attrs-start title = 'QuestionResult 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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