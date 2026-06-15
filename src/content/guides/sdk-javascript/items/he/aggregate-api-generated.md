מאגד מסמכים על ידי קיבוצם (אם מסופק groupBy) ויישום מספר פעולות. נתמכות פעולות שונות (למשל sum, countDistinct, avg וכו').

## פרמטרים

| שם | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| aggregationRequest | AggregationRequest | כן |  |
| parentTenantId | string | לא |  |
| includeStats | boolean | לא |  |

## תגובה

מחזיר: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-aggregate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_78a9';
const parentTenantId: string = 'parent_tenant_01';
const includeStats: boolean = true;
const aggregationRequest: AggregationRequest = {
  operation: { type: 'COUNT' },
  groupBy: ['pageUrl'],
  predicate: { field: 'status', operator: 'EQUALS', value: 'approved' },
  sort: [{ field: 'count', direction: 'DESC' }],
  limit: 25
};
const result: Aggregate200Response = await aggregate(tenantId, aggregationRequest, parentTenantId, includeStats);
[inline-code-end]

---