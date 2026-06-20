מאגדת מסמכים על ידי קיבוצם (אם מסופק groupBy) ויישום מספר פעולות.
תומכת בפעולות שונות (למשל sum, countDistinct, avg וכו').

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| parentTenantId | string | query | לא |  |
| includeStats | boolean | query | לא |  |

## תגובה

מחזיר: [`AggregateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-aggregate'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. עבור כל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let aggregationRequest = AggregationRequest(query: [QueryPredicate(key: "key_example", value: QueryPredicate_value(), _operator: "_operator_example")], resourceName: "resourceName_example", groupBy: ["groupBy_example"], operations: [AggregationOperation(field: "field_example", op: AggregationOpType(), alias: "alias_example", expandArray: false)], sort: AggregationRequest_sort(dir: "dir_example", field: "field_example")) // AggregationRequest | 
let parentTenantId = "parentTenantId_example" // String |  (אופציונלי)
let includeStats = true // Bool |  (אופציונלי)

DefaultAPI.aggregate(tenantId: tenantId, aggregationRequest: aggregationRequest, parentTenantId: parentTenantId, includeStats: includeStats) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---