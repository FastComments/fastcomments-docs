Um Ergebnisse für Fragen zu speichern, erstellen Sie ein `QuestionResult`. Sie können dann Frageergebnisse aggregieren und sie auch
zu Berichtszwecken mit Kommentaren verknüpfen.

[inline-code-attrs-start title = 'QuestionResult Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
