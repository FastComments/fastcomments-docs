[api-resource-header-start name = 'Aggregate'; route = 'GET /api/v1/aggregate'; creditsCost = 1; api-resource-header-end]

Ovaj API agregira dokumente grupiranjem (ako je naveden groupBy) i primjenom više operacija.
Podržane su različite operacije (npr. sum, countDistinct, avg, itd.).

Trošak je **varijabilan**. Svakih 500 pregledanih objekata košta 1 API kredit.

Maksimalna dopuštena upotreba memorije po API pozivu po zadanim postavkama iznosi 64MB, a po zadanim postavkama možete imati samo jednu agregaciju koja se izvršava u jednom trenutku. Ako istodobno pošaljete više agregacija, bit će stavljenje u red i izvršene redoslijedom podnošenja. Neobrađene agregacije čekat će najviše 60 sekundi, nakon čega će zahtjev isteći. Pojedinačne agregacije mogu se izvršavati do 5 minuta.

Ako imate upravljane tenante, možete agregirati sve resurse podređenih tenant-a u jednom pozivu prosljeđivanjem query parametra `parentTenantId`.

## Primjeri

### Primjer: Brojanje jedinstvenih

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: broj jedinstvenih vrijednosti'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Odgovor: broj jedinstvenih vrijednosti'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Primjer: Brojanje različitih vrijednosti

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: broj različitih vrijednosti'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "countDistinct", "field": "urlId", "alias": "urlId" },
        { "op": "countDistinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

Odgovor:

[inline-code-attrs-start title = 'Odgovor: broj različitih vrijednosti'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Primjer: Zbroj vrijednosti više polja

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: zbroj vrijednosti'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "sum", "field": "votes", "alias": "votes" },
        { "op": "sum", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

Odgovor:

[inline-code-attrs-start title = 'Odgovor: zbroj vrijednosti'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Primjer: Prosjek vrijednosti više polja

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: prosjek vrijednosti'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "avg", "field": "votes", "alias": "votes" },
        { "op": "avg", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

Odgovor:

[inline-code-attrs-start title = 'Odgovor: prosjek vrijednosti'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Primjer: Min/Max vrijednosti više polja

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: min/max vrijednosti'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Odgovor:

[inline-code-attrs-start title = 'Odgovor: min/max vrijednosti'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Primjer: Brojanje jedinstvenih vrijednosti više polja

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: broj jedinstvenih vrijednosti'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

Odgovor:

[inline-code-attrs-start title = 'Odgovor: broj jedinstvenih vrijednosti'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Primjer: Kreiranje upita

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: kreiranje upita'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Odgovor:

[inline-code-attrs-start title = 'Odgovor: kreiranje upita'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Primjer: Brojanje komentara na čekanju pregleda

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: broj komentara na čekanju pregleda'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Odgovor:

[inline-code-attrs-start title = 'Odgovor: broj komentara na čekanju pregleda'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        { "count": { "numericValue": 2 } }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### Primjer: Pregled raspodjele odobrenih, pregledanih i spam komentara

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: pregled raspodjele komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Odgovor:

[inline-code-attrs-start title = 'Odgovor: pregled raspodjele komentara'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Strukture

[inline-code-attrs-start title = 'Struktura zahtjeva za agregaciju'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export type AggregationOpType = 'sum' | 'countDistinct' | 'distinct' | 'avg' | 'min' | 'max' | 'count';

/** Operacija koja će se primijeniti na polje */
export interface AggregationOperation {
    /** Polje na kojem će se izvršiti operacija */
    field: string;
    /** Tip operacije */
    op: AggregationOpType;
    /** Opcionalni alias za izlaz; ako nije naveden, koristi se izračunati zadani alias */
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

[inline-code-attrs-start title = 'Struktura odgovora za agregaciju'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Sljedeći resursi mogu se agregirati:

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