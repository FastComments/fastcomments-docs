סטטוס, התקדמות ו, לאחר סיום, התוצאה של משימת אצווה אחת.

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## Response

Returns: [`GetBatchJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמת getBatchJob'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.getBatchJob(tenantId: tenantId, id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]