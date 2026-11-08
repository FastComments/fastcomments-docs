FastComments प्रश्न बनाने और उनके परिणामों को संग्रहित (aggregate) करने का एक तरीका प्रदान करता है। एक प्रश्न (आगे इसे `QuestionConfig` कहा जाएगा) का उदाहरण
एक स्टार रेटिंग, एक स्लाइडर, या एक NPS प्रश्न हो सकता है (`type` के माध्यम से निर्धारित)।

प्रश्न डेटा को अलग-अलग, साथ में, समय के साथ, समग्र रूप से, पेज के अनुसार आदि तरीकों से संग्रहित किया जा सकता है।

इस फ्रेमवर्क में क्लाइंट-साइड विजेट (इस API के सामने आपके सर्वर के साथ), एडमिन डैशबोर्ड, और रिपोर्टिंग टूल बनाने के लिए आवश्यक सभी क्षमताएं मौजूद हैं।

सबसे पहले, हमें एक `QuestionConfig` परिभाषित करना होगा। इसकी संरचना इस प्रकार है:

[inline-code-attrs-start title = 'QuestionConfig संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type QuestionConfigType = 'nps' | 'slider' | 'star' | 'thumbs';

interface QuestionConfig {
    id: string
    tenantId: string
    name: string
    question: string
    helpText?: string
    createdAt: string
    createdBy: string
    /** READONLY - incremented for each new response. **/
    usedCount: number
    /** A date string for when the configuration was last used (result left). **/
    lastUsed?: string
    type: QuestionConfigType
    numStars?: number
    min?: number
    max?: number
    defaultValue?: number
    labelNegative?: string
    labelPositive?: string
    subQuestionIds?: string[]
    alwaysShowSubQuestions?: boolean
    reportingOrder: number
}
[inline-code-end]