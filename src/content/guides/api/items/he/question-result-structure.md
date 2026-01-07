כדי לשמור תוצאות לשאלות, אתה יוצר `QuestionResult`. לאחר מכן תוכל לאגור תוצאות שאלות, וגם
לקשר אותן לתגובות למטרות דיווח.

[inline-code-attrs-start title = 'מבנה QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
