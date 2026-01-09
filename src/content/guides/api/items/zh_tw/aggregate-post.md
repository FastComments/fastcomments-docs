[api-resource-header-start name = 'Aggregate'; route = 'GET /api/v1/aggregate'; creditsCost = 1; api-resource-header-end]

此 API 會對文件進行彙總（若提供 groupBy 則會先分群），並套用多個運算。支援不同的運算（例如 sum、countDistinct、avg 等）。

費用為 **可變**。每掃描 500 個物件會耗費 1 個 API 點數。

預設每次 API 呼叫可使用的最大記憶體為 64MB，並且預設同一時間只允許一個彙總在執行。若同時提交多個彙總，將會排入佇列並依提交順序執行。待處理的彙總最多會等待 60 秒，超過後請求將會逾時。單一彙總最多可執行 5 分鐘。

若您有管理的租戶，可在一次呼叫中透過傳遞 `parentTenantId` 查詢參數來彙總所有子租戶的資源。

## 範例

### 範例：計算唯一值

[inline-code-attrs-start title = '計算唯一值 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = '計算唯一值 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 範例：計算不同值

[inline-code-attrs-start title = '計算不同值 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "countDistinct", "field": "urlId", "alias": "urlId" },
        { "op": "countDistinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

Response:

[inline-code-attrs-start title = '計算不同值 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 範例：多欄位加總值

[inline-code-attrs-start title = '加總值 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "sum", "field": "votes", "alias": "votes" },
        { "op": "sum", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

Response:

[inline-code-attrs-start title = '加總值 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 範例：多欄位平均值

[inline-code-attrs-start title = '平均值 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "avg", "field": "votes", "alias": "votes" },
        { "op": "avg", "field": "votesUp", "alias": "votesUp" }
    ]
}'
[inline-code-end]

Response:

[inline-code-attrs-start title = '平均值 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 範例：多欄位最小/最大值

[inline-code-attrs-start title = '最小/最大值 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Response:

[inline-code-attrs-start title = '最小/最大值 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 範例：多欄位唯一值計數

[inline-code-attrs-start title = '計算唯一值 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST --url 'https://fastcomments.com/api/v1/aggregate?tenantId=demo&API_KEY=DEMO_API_SECRET&includeStats=true' --header 'Content-Type: application/json' --data '{
    "resourceName": "Comment",
    "operations": [
        { "op": "distinct", "field": "urlId", "alias": "urlId" },
        { "op": "distinct", "field": "commenterEmail", "alias": "commenterEmail" }
    ]
}'
[inline-code-end]

Response:

[inline-code-attrs-start title = '計算唯一值 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 範例：建立查詢

[inline-code-attrs-start title = '建立查詢 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Response:

[inline-code-attrs-start title = '建立查詢 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 範例：計算待審核評論

[inline-code-attrs-start title = '計算待審核評論 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Response:

[inline-code-attrs-start title = '計算待審核評論 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "data": [
        { "count": { "numericValue": 2 } }
    ],
    "stats": { "scanned": 2 }
}
[inline-code-end]

### 範例：已核准、已審核與垃圾評論的分類統計

[inline-code-attrs-start title = '評論分類統計 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Response:

[inline-code-attrs-start title = '評論分類統計 回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 結構

[inline-code-attrs-start title = '彙總請求 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export type AggregationOpType = 'sum' | 'countDistinct' | 'distinct' | 'avg' | 'min' | 'max' | 'count';

/** 將套用於欄位的運算 */
export interface AggregationOperation {
    /** 要操作的欄位 */
    field: string;
    /** 運算的類型 */
    op: AggregationOpType;
    /** 輸出的可選別名；若未提供會計算預設別名 */
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

[inline-code-attrs-start title = '彙總回應 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

The following resources can be aggregated:

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