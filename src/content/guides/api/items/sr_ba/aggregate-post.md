[api-resource-header-start name = 'Aggregate'; route = 'GET /api/v1/aggregate'; creditsCost = 1; api-resource-header-end]

Овај API агрегира документе групишући их (ако је groupBy наведен) и примјењујући више операција.
Подржане су различите операције (нпр. sum, countDistinct, avg, итд).

Трошак је **променљиво**. Сваких 500 скенираних објеката кошта 1 API кредит.

Максимална меморија дозвољена по API позиву по подразумеваној поставци је 64MB, и по подразумеваној поставци можете имати само једну агрегaцију која се извршава у једном тренутку. Ако пошаљете више агрегaција истовремено, биће стављене у ред и извршаваће се по редослиједу слања. Очекујуће агрегaције ће чекати највише 60 секунди, након чега ће захтјев истeћи. Појединачне агрегaције могу трајати до 5 минута.

Ако имате managed tenants, можете агрегирати све child tenant ресурсе у једном позиву прослеђивањем `parentTenantId` query параметра.

## Примјери

### Примјер: Бројање јединствених

[inline-code-attrs-start title = 'cURL пример: Бројање јединствених вредности'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Одговор: Бројање јединствених вредности'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Примјер: Бројање различитих

[inline-code-attrs-start title = 'cURL пример: Бројање различитих вредности'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "countDistinct", "field": "urlId", "alias": "urlId" },
        { "op": "countDistinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

Одговор:

[inline-code-attrs-start title = 'Одговор: Бројање различитих вредности'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Примјер: Сума вриједности више поља

[inline-code-attrs-start title = 'cURL пример: Сума вредности'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "sum", "field": "votes", "alias": "votes" },
        { "op": "sum", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

Одговор:

[inline-code-attrs-start title = 'Одговор: Сума вредности'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Примјер: Просјечне вриједности више поља

[inline-code-attrs-start title = 'cURL пример: Просјечне вриједности'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "avg", "field": "votes", "alias": "votes" },
        { "op": "avg", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

Одговор:

[inline-code-attrs-start title = 'Одговор: Просјечне вриједности'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Примјер: Минималне/Максималне вриједности више поља

[inline-code-attrs-start title = 'cURL пример: Минималне/Максималне вредности'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Одговор:

[inline-code-attrs-start title = 'Одговор: Минималне/Максималне вредности'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Примјер: Бројање јединствених вриједности више поља

[inline-code-attrs-start title = 'cURL пример: Бројање јединствених вредности'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

Одговор:

[inline-code-attrs-start title = 'Одговор: Бројање јединствених вредности'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Примјер: Креирање упита

[inline-code-attrs-start title = 'cURL пример: Креирање упита'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Одговор:

[inline-code-attrs-start title = 'Одговор: Креирање упита'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Примјер: Бројање коментара који чекају рецензију

[inline-code-attrs-start title = 'cURL пример: Бројање коментара на чекању рецензије'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Одговор:

[inline-code-attrs-start title = 'Одговор: Бројање коментара на чекању рецензије'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        { "count": { "numericValue": 2 } }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### Примјер: Преглед по статусу (approved, reviewed, spam)

[inline-code-attrs-start title = 'cURL пример: Преглед коментара по статусу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Одговор:

[inline-code-attrs-start title = 'Одговор: Преглед коментара по статусу'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Структуре

[inline-code-attrs-start title = 'Структура Aggregate захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export type AggregationOpType = 'sum' | 'countDistinct' | 'distinct' | 'avg' | 'min' | 'max' | 'count';

/** Операција која ће се извршити над пољем */
export interface AggregationOperation {
    /** Поље над којим се извршава операција */
    field: string;
    /** Тип операције */
    op: AggregationOpType;
    /** Опциони алијас за излаз; ако није наведен, израчунава се подразумевани алијас */
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

[inline-code-attrs-start title = 'Структура Aggregate одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Следећи ресурси се могу агрегирати:

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