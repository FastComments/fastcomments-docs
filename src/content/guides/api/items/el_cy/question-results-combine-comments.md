[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Εδώ γίνεται ο συνδυασμός αποτελεσμάτων με σχόλια. Χρήσιμο για τη δημιουργία ενός γραφήματος "πρόσφατα θετικά και αρνητικά σχόλια" για ένα προϊόν, για παράδειγμα.

Μπορείτε να αναζητήσετε μέσω ενός εύρους τιμών (συμπεριλαμβανόμενων), μιας ή περισσότερων ερωτήσεων, και με αρχική ημερομηνία (συμπεριλαμβανόμενη).

Η δομή απάντησης είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή QuestionResultsCombinedWithCommentsResponse'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Εδώ είναι οι διαθέσιμες παράμετροι ερωτήματος για συγκέντρωση:

[inline-code-attrs-start title = 'Δομή Αιτήματος QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Εδώ είναι ένα παράδειγμα αιτήματος:

[inline-code-attrs-start title = 'Παράδειγμα QuestionResultsCombineComments'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Παράδειγμα απάντησης:

[inline-code-attrs-start title = 'Παράδειγμα Απάντησης QuestionResultsCombineComments'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Δομή Απάντησης QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Σημειώσεις Προσωρινής Αποθήκευσης και Κόστους

- Όταν καθορίζεται το `forceRecalculate`, το κόστος είναι πάντα `10`, αντί του κανονικού `2`.
- Αν η προσωρινή μνήμη λήξει και τα δεδομένα επανυπολογιστούν, το κόστος παραμένει σταθερό `2` αν δεν καθοριστεί το `forceRecalculate`.
- Αυτό γίνεται για να ενθαρρύνει τη χρήση της προσωρινής μνήμης.
