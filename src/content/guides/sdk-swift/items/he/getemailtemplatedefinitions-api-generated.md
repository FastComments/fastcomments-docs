## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplateDefinitions200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getEmailTemplateDefinitions'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לגבי כל בעיה, נא לדווח באמצעות http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getEmailTemplateDefinitions(tenantId: tenantId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]