העלאת תמונה ושינוי גודלה

## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | פרמטר גודל: \"Default\" (1000x1000px) או \"CrossPlatform\" (יוצר גדלים למכשירים פופולריים) |
| urlId | string | query | No | מזהה דף שממ ממנו מתבצע ההעלאה, לתצורה |

## תגובה

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגמי הקוד הבאים עדיין בתוכנה ניסיונית. עבור כל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | פרמטר גודל: \"Default\" (1000x1000px) או \"CrossPlatform\" (יוצר גדלים למכשירים פופולריים) (אופציונלי)
let urlId = "urlId_example" // String | מזהה דף שממ ממנו מתבצע ההעלאה, לתצורה (אופציונלי)

PublicAPI.uploadImage(tenantId: tenantId, file: file, options: PublicAPI.UploadImageOptions(sizePreset: sizePreset, urlId: urlId)) { (response, error) in
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