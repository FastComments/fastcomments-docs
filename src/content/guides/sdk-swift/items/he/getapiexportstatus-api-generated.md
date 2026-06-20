## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| batchJobId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportStatusResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getApiExportStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמי הקוד הבאים עדיין בבטא. לכל תקלה, אנא דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let batchJobId = "batchJobId_example" // String |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.getApiExportStatus(batchJobId: batchJobId, sso: sso) { (response, error) in
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