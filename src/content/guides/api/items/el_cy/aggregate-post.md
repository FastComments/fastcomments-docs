[api-resource-header-start name = 'Aggregate'; route = 'GET /api/v1/aggregate'; creditsCost = 1; api-resource-header-end]

Αυτό το API συγκεντρώνει έγγραφα ομαδοποιώντας τα (αν παρέχεται groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.
Υποστηρίζονται διαφορετικές λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.).

Το κόστος είναι **μεταβλητό**. Κάθε 500 αντικείμενα που σαρώνονται κοστίζουν 1 API credit.

Η μέγιστη χρήση μνήμης που επιτρέπεται ανά κλήση API από προεπιλογή είναι 64MB, και από προεπιλογή μπορείτε να έχετε μόνο μία συγκέντρωση να εκτελείται τη φορά. Αν υποβάλετε πολλαπλές
συγκεντρώσεις ταυτόχρονα, θα μπουν σε ουρά και θα εκτελεστούν με τη σειρά που υποβλήθηκαν. Οι εκκρεμείς συγκεντρώσεις θα περιμένουν το πολύ 60 δευτερόλεπτα, μετά από αυτό
το αίτημα θα λήξει. Οι μεμονωμένες συγκεντρώσεις μπορούν να εκτελούνται για έως 5 λεπτά.

Αν έχετε διαχειριζόμενους tenants, μπορείτε να συγκεντρώσετε όλους τους πόρους των θυγατρικών tenants σε μία κλήση περνώντας την παράμετρο query `parentTenantId`.

## Παραδείγματα

### Παράδειγμα: Μέτρηση Μοναδικών

[inline-code-attrs-start title = 'Μέτρηση Μοναδικών Τιμών cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Μέτρηση Μοναδικών Τιμών Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Παράδειγμα: Μέτρηση Διακριτών

[inline-code-attrs-start title = 'Μέτρηση Διακριτών Τιμών cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "countDistinct", "field": "urlId", "alias": "urlId" },
        { "op": "countDistinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

Απάντηση:

[inline-code-attrs-start title = 'Μέτρηση Διακριτών Τιμών Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Παράδειγμα: Άθροισμα Τιμών Πολλαπλών Πεδίων

[inline-code-attrs-start title = 'Άθροισμα Τιμών cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "sum", "field": "votes", "alias": "votes" },
        { "op": "sum", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

Απάντηση:

[inline-code-attrs-start title = 'Άθροισμα Τιμών Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Παράδειγμα: Μέσος Όρος Τιμών Πολλαπλών Πεδίων

[inline-code-attrs-start title = 'Μέσος Όρος Τιμών cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "avg", "field": "votes", "alias": "votes" },
        { "op": "avg", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

Απάντηση:

[inline-code-attrs-start title = 'Μέσος Όρος Τιμών Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Παράδειγμα: Ελάχιστο/Μέγιστο Τιμών Πολλαπλών Πεδίων

[inline-code-attrs-start title = 'Ελάχιστο/Μέγιστο Τιμών cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Απάντηση:

[inline-code-attrs-start title = 'Ελάχιστο/Μέγιστο Τιμών Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Παράδειγμα: Μέτρηση Μοναδικών Τιμών Πολλαπλών Πεδίων

[inline-code-attrs-start title = 'Μέτρηση Μοναδικών Τιμών cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

Απάντηση:

[inline-code-attrs-start title = 'Μέτρηση Μοναδικών Τιμών Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Παράδειγμα: Δημιουργία Ερωτήματος

[inline-code-attrs-start title = 'Δημιουργία Ερωτήματος cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Απάντηση:

[inline-code-attrs-start title = 'Δημιουργία Ερωτήματος Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Παράδειγμα: Μέτρηση Σχολίων σε Αναμονή Ελέγχου

[inline-code-attrs-start title = 'Μέτρηση Σχολίων σε Αναμονή Ελέγχου cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Απάντηση:

[inline-code-attrs-start title = 'Μέτρηση Σχολίων σε Αναμονή Ελέγχου Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        { "count": { "numericValue": 2 } }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### Παράδειγμα: Ανάλυση Εγκεκριμένων, Ελεγμένων και Spam Σχολίων

[inline-code-attrs-start title = 'Ανάλυση Σχολίων cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Απάντηση:

[inline-code-attrs-start title = 'Ανάλυση Σχολίων Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Δομές

[inline-code-attrs-start title = 'Δομή Αιτήματος Συγκέντρωσης'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Δομή Απάντησης Συγκέντρωσης'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Οι ακόλουθοι πόροι μπορούν να συγκεντρωθούν:

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
