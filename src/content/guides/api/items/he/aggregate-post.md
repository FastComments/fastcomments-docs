[api-resource-header-start name = 'Aggregate'; route = 'GET /api/v1/aggregate'; creditsCost = 1; api-resource-header-end]

API זה מצבר מסמכים על ידי קיבוצם (אם groupBy מסופק) והחלת פעולות מרובות.
פעולות שונות (למשל sum, countDistinct, avg וכו') נתמכות.

העלות היא **משתנה**. כל 500 אובייקטים שנסרקים עולים 1 קרדיט API.

שימוש הזיכרון המקסימלי המותר לקריאת API היא 64MB כברירת מחדל, וכברירת מחדל מותר לך להריץ רק צבירה אחת בכל פעם. אם תשלח מספר
צבירות בו-זמנית, הן יתווספו לתור ויורצו בסדר שהוגשו. צבירות ממתינות יחכו מקסימום 60 שניות, לאחר מכן
הבקשה תפוג. צבירות בודדות יכולות לרוץ עד 5 דקות.

אם יש לך שוכרים מנוהלים, אתה יכול לצבור את כל משאבי שוכרי הילדים בקריאה אחת על ידי העברת פרמטר השאילתה `parentTenantId`.

## דוגמאות

### דוגמה: ספירת ייחודיים

[inline-code-attrs-start title = 'דוגמת cURL לספירת ערכים ייחודיים'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'תגובת ספירת ערכים ייחודיים'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        {
            "commenterEmail": {
                "distinctCounts": {
                    "someone@somewhere.com": 1,
                    "someone2@somewhere.com": 1
                }
            }
        },
        {
            "urlId": {
                "distinctCounts": {
                    "some-page": 2
                }
            }
        }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### דוגמה: ספירת נבדלים

[inline-code-attrs-start title = 'דוגמת cURL לספירת ערכים נבדלים'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "countDistinct", "field": "urlId", "alias": "urlId" },
        { "op": "countDistinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

תגובה:

[inline-code-attrs-start title = 'תגובת ספירת ערכים נבדלים'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        {
            "commenterEmail": { "distinctCount": 2 },
            "urlId": { "distinctCount": 1 }
        }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### דוגמה: סיכום ערכים של שדות מרובים

[inline-code-attrs-start title = 'דוגמת cURL לסיכום ערכים'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "sum", "field": "votes", "alias": "votes" },
        { "op": "sum", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

תגובה:

[inline-code-attrs-start title = 'תגובת סיכום ערכים'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        {
            "votes": { "numericValue": 2 },
            "votesUp": { "numericValue": 2 }
        }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### דוגמה: ממוצע ערכים של שדות מרובים

[inline-code-attrs-start title = 'דוגמת cURL לממוצע ערכים'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "avg", "field": "votes", "alias": "votes" },
        { "op": "avg", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

תגובה:

[inline-code-attrs-start title = 'תגובת ממוצע ערכים'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        {
            "votes": { "numericValue": 1 },
            "votesUp": { "numericValue": 1 }
        }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### דוגמה: ערכי מינימום/מקסימום של שדות מרובים

[inline-code-attrs-start title = 'דוגמת cURL לערכי מינימום/מקסימום'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "min", "field": "votes", "alias": "votes" },
        { "op": "min", "field": "votesUp", "alias": "votesUp" },
        { "op": "max", "field": "votes", "alias": "votes" },
        { "op": "max", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

תגובה:

[inline-code-attrs-start title = 'תגובת ערכי מינימום/מקסימום'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        {
            "votes": { "numericValue": 0 },
            "votesUp": { "numericValue": 0 },
            "votes": { "numericValue": 2 },
            "votesUp": { "numericValue": 2 }
        }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### דוגמה: ספירת ערכים ייחודיים של שדות מרובים

[inline-code-attrs-start title = 'דוגמת cURL לספירת ערכים ייחודיים'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

תגובה:

[inline-code-attrs-start title = 'תגובת ספירת ערכים ייחודיים'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        {
            "commenterEmail": {
                "distinctCounts": {
                    "someone@somewhere.com": 1,
                    "someone2@somewhere.com": 1
                }
            },
            "urlId": {
                "distinctCounts": {
                    "some-page": 2
                }
            }
        }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### דוגמה: יצירת שאילתה

[inline-code-attrs-start title = 'דוגמת cURL ליצירת שאילתה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "groupBy": ["commenterName"],
    "query": [
        { "key": "approved", "value": true, "operator": "eq" },
        { "key": "commenterName", "value": "some-username-2", "operator": "eq" }
    ],
    "operations": [
        { "op": "sum", "field": "votes", "alias": "votes" }
    ]
}'
[inline-code-end]

תגובה:

[inline-code-attrs-start title = 'תגובת יצירת שאילתה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        {
            "groups": { "commenterName": "some-username-2" },
            "votes": { "numericValue": 2 }
        }
    ],
    "stats": { "scanned": 1 }
}
[inline-code-end]

### דוגמה: ספירת תגובות הממתינות לבדיקה

[inline-code-attrs-start title = 'דוגמת cURL לספירת תגובות הממתינות לבדיקה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "query": [
        { "key": "reviewed", "value": true, "operator": "not_eq" }
    ],
    "operations": [
        { "op": "count", "field": "id", "alias": "count" }
    ]
}'
[inline-code-end]

תגובה:

[inline-code-attrs-start title = 'תגובת ספירת תגובות הממתינות לבדיקה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        { "count": { "numericValue": 2 } }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### דוגמה: פירוט תגובות מאושרות, נבדקות וספאם

[inline-code-attrs-start title = 'דוגמת cURL לפירוט תגובות'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "approved", "alias": "approved" },
        { "op": "distinct", "field": "reviewed", "alias": "reviewed" },
        { "op": "distinct", "field": "isSpam", "alias": "isSpam" }
    ]
}'
[inline-code-end]

תגובה:

[inline-code-attrs-start title = 'תגובת פירוט תגובות'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        {
            "approved": { "distinctCounts": { "true": 2 } },
            "reviewed": { "distinctCounts": { "false": 2 } },
            "isSpam": { "distinctCounts": { "false": 2 } }
        }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### מבנים

[inline-code-attrs-start title = 'מבנה בקשת צבירה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export type AggregationOpType = 'sum' | 'countDistinct' | 'distinct' | 'avg' | 'min' | 'max' | 'count';

/** An operation that will be applied on a field */
export interface AggregationOperation {
    /** The field to operate on */
    field: string;
    /** The type of operation */
    op: AggregationOpType;
    /** Optional alias for the output; if not provided, a default alias is computed */
    alias?: string;
    expandArray?: boolean;
}

export interface QueryPredicate {
    key: string
    value: string | number | boolean
    operator: 'eq' | 'not_eq' | 'greater_than' | 'less_than' | 'contains'
}

export interface AggregationRequest {
    query?: QueryPredicate[];
    resourceName: string;
    groupBy?: string[];
    operations: AggregationOperation[];
    sort?: {
        field: string;
        dir: 'asc' | 'desc';
    };
}

type DistinctAccumulator = Record<string, number>;
type GroupValues = Record<string, string>;

export type AggregationValue = {
    distinctCounts?: DistinctAccumulator
    distinctCount?: number
    numericValue?: number
    stringValue?: string
    groups?: GroupValues
};
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת צבירה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export type AggregationItem = Record<string, AggregationValue> & { groups?: GroupValues };

export interface AggregationResponse {
    status: APIStatus,
    data: AggregationItem[];
    stats?: {
        scanned: number;
        timeMS: number;
    };
}
[inline-code-end]

המשאבים הבאים ניתנים לצבירה:

- AffiliateEvent
- AnonymousVote
- BannedUser
- BatchJob
- BlockedUser
- Comment
- CommentDeleted
- CommentIdToSyncOutbound
- CommentScheduled
- CommentSyncLog
- CustomConfig
- CustomEmailTemplateRenderError
- EmailToSend
- EventLogEntry
- ImportedCommentScheduled
- ModerationGroup
- Moderator
- Page
- PageReact
- PendingVote
- QuestionResult
- SSOUser
- SentEmail
- SpamEvent
- Tenant
- TenantAuditLog
- TenantBadge
- TenantDailyUsage
- TenantInvoiceHistory
- TenantPackage
- User
- UserBadge
- UserBadgeProgress
- UserNotification
- UserSubscription
- UserUsage
- Vote
