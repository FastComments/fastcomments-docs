מאגדת מסמכים על ידי קיבוצם (אם נמסר groupBy) והחלת מספר פעולות. נתמכות פעולות שונות (למשל sum, countDistinct, avg וכו').

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## תגובה

מחזיר: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-aggregate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_72b3";
const parentTenantId: string = "parent_acme_corp";
const aggregationRequest: AggregationRequest = {
  groupBy: ["postId"],
  predicates: [
    { field: "status", operator: "EQ", value: { stringValue: "published" } as QueryPredicateValue }
  ],
  operations: [
    { type: AggregationOpType.COUNT, field: "id", alias: "commentCount" } as AggregationOperation
  ],
  sort: [{ field: "commentCount", direction: "DESC" } as AggregationRequestSort],
  limit: 25
};
const response: AggregateResponse = await aggregate(tenantId, aggregationRequest, parentTenantId, true);
[inline-code-end]

---