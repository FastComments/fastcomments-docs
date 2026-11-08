प्रश्नों के परिणाम सहेजने के लिए, आप एक `QuestionResult` बनाते हैं। इसके बाद आप प्रश्न परिणामों को एग्रीगेट कर सकते हैं, और रिपोर्टिंग उद्देश्यों के लिए उन्हें टिप्पणियों से भी जोड़ सकते हैं।

[inline-code-attrs-start title = 'QuestionResult संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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