The tenant's batch jobs (imports, exports, page migrations...), החדשות ביותר ראשונות.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | רק עבודות מהסוג הזה, לדוגמה migrate-pages. |
| status | string | query | No | רק עבודות במצב זה: requested, running, success or failed. |
| skip | integer | query | No | כמה עבודות לדלג, עבור דפדוף. |

## תגובה

מחזיר: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'getBatchJobs דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הקוד הבא עדיין בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | רק עבודות מהסוג הזה, לדוגמה migrate-pages. (אופציונלי)
let status = "status_example" // String | רק עבודות במצב זה: requested, running, success or failed. (אופציונלי)
let skip = 987 // Int | כמה עבודות לדלג, עבור דפדוף. (אופציונלי)

DefaultAPI.getBatchJobs(tenantId: tenantId, options: DefaultAPI.GetBatchJobsOptions(type: type, status: status, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]