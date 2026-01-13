Για να αποθηκεύσετε αποτελέσματα για ερωτήσεις, δημιουργείτε ένα `QuestionResult`. Μπορείτε στη συνέχεια να συγκεντρώσετε τα αποτελέσματα ερωτήσεων, και επίσης
να τα συνδέσετε με σχόλια για σκοπούς αναφοράς.

[inline-code-attrs-start title = 'Δομή QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
