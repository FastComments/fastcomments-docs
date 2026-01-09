מאחד מסמכים על ידי קיבוצם (אם נמסר groupBy) ויישום מספר פעולות. נתמכות פעולות שונות (למשל sum, countDistinct, avg וכו').

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| aggregationRequest | AggregationRequest | כן |  |
| parentTenantId | string | לא |  |
| includeStats | boolean | לא |  |

## תגובה

מחזיר: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---