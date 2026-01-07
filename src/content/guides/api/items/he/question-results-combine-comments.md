[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

כאן מתרחש שילוב תוצאות עם תגובות. שימושי ליצירת תרשים "תגובות חיוביות ושליליות אחרונות" למוצר, לדוגמה.

אתה יכול לחפש באמצעות טווח ערכים (כולל), שאלה אחת או יותר, ולפי תאריך התחלה (כולל).

מבנה התגובה הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה QuestionResultsCombinedWithCommentsResponse'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SimpleComment {
    id: string
    commenterName: string
    userId?: string
    date: string
    commentHTML: string
}

interface SimpleQuestionResult {
    id: string
    value: number
}

interface CommentAndResult {
    comment: SimpleComment
    result: SimpleQuestionResult
}

interface QuestionResultsCombinedWithCommentsResponse {
    /** A date string representing when this data was calculated, since it might come from cache. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

הנה פרמטרי השאילתה הזמינים לאיגום:

[inline-code-attrs-start title = 'מבנה בקשת QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** You can aggregate results for one or more questions. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

הנה דוגמת בקשה:

[inline-code-attrs-start title = 'דוגמת QuestionResultsCombineComments'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

דוגמת תגובה:

[inline-code-attrs-start title = 'דוגמת תגובת QuestionResultsCombineComments'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "createdAt": "2023-08-30T00:00:00.000Z",
    "results": [
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-2",
                "commenterName": "Test",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 10
            }
        },
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-0",
                "commenterName": "Some Name",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 5
            }
        }
    ]
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### הערות על מטמון ועלות

- כאשר `forceRecalculate` מצוין, העלות היא תמיד `10`, במקום `2` הרגיל.
- אם המטמון פג ונתונים מחושבים מחדש, העלות היא עדיין `2` קבוע אם `forceRecalculate` לא מצוין.
- זה כדי לתמרץ שימוש במטמון.
