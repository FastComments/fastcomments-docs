FastComments מספק דרך לבנות שאלות ולאגור את תוצאותיהן. דוגמה לשאלה (להלן נקראת `QuestionConfig`)
יכולה להיות דירוג כוכבים, מחוון, או שאלת NPS (נקבע באמצעות `type`).

ניתן לאגור נתוני שאלות בנפרד, יחד, לאורך זמן, באופן כולל, לפי עמוד, וכן הלאה.

למסגרת יש את כל היכולות הנדרשות לבניית ווידג'טים בצד הלקוח (עם השרת שלך מול API זה), דשבורדים למנהלים, וכלי דיווח.

ראשית, עלינו להגדיר `QuestionConfig`. המבנה הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
