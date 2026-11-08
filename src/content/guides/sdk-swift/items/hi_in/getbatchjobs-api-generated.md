टेनेंट की बैच जॉब्स (imports, exports, page migrations...), नवीनतम पहले।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| type | string | query | नहीं | केवल इस प्रकार की जॉब्स, उदाहरण के लिए migrate-pages। |
| status | string | query | नहीं | केवल इस स्थिति में जॉब्स: requested, running, success या failed। |
| skip | integer | query | नहीं | पेजिंग के लिए कितनी जॉब्स छोड़नी हैं। |

## प्रतिक्रिया

रिटर्न करता है: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getBatchJobs उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | केवल इस प्रकार की जॉब्स, उदाहरण के लिए migrate-pages। (optional)
let status = "status_example" // String | केवल इस स्थिति में जॉब्स: requested, running, success या failed। (optional)
let skip = 987 // Int | पेजिंग के लिए कितनी जॉब्स छोड़नी हैं। (optional)

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