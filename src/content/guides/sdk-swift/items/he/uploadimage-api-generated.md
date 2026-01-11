העלאה ושינוי גודל של תמונה

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| sizePreset | string | query | לא | הגדרת גודל: "Default" (1000x1000px) או "CrossPlatform" (יוצר גדלים למכשירים פופולריים) |
| urlId | string | query | לא | מזהה הדף שממנו מתבצעת ההעלאה, לצורך תצורה |

## תגובה

מחזיר: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | הגדרת גודל: \"Default\" (1000x1000px) או \"CrossPlatform\" (יוצר גדלים למכשירים פופולריים) (אופציונלי)
let urlId = "urlId_example" // String | מזהה הדף שממנו מתבצעת ההעלאה, לצורך תצורה (אופציונלי)

PublicAPI.uploadImage(tenantId: tenantId, file: file, sizePreset: sizePreset, urlId: urlId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]