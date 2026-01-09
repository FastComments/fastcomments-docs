FastComments надає спосіб створювати запитання та агрегувати їхні результати. Приклад запитання (надалі — `QuestionConfig`) може бути рейтингом у вигляді зірок, повзунком або питанням NPS (визначається через `type`).

Дані запитання можна агрегувати окремо, разом, з часом, загалом, за сторінкою тощо.

Фреймворк має всі можливості, необхідні для створення клієнтських віджетів (з вашим сервером перед цим API), панелей адміністратора та інструментів звітності.

Спочатку потрібно визначити `QuestionConfig`. Структура виглядає так:

[inline-code-attrs-start title = 'Структура QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** ТІЛЬКИ ДЛЯ ЧИТАННЯ - збільшується для кожної нової відповіді. **/
    usedCount: number
    /** Рядок дати, коли конфігурація була востаннє використана (залишено результат). **/
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


---