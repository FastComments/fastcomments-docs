एक बैच जॉब कतार में जोड़ता है जो एक migrate-pages जॉब के हर माइग्रेशन को उल्टा कर देता है, नवीनतम पहले। 90 दिनों तक उपलब्ध।
माइग्रेशन के बाद बदले गए रिकॉर्ड को नहीं छेड़ा जाता, इसलिए बाद का कोई संपादन कभी अधिलेखित नहीं होता।
पुनर्स्थापित/छोड़े गए की गिनती के लिए GET /api/v1/batch-jobs/{batchJobId} को पोल करें।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| undoId | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'undoPageMigrations उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let undoId = "undoId_example" // String | 

DefaultAPI.undoPageMigrations(tenantId: tenantId, undoId: undoId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]