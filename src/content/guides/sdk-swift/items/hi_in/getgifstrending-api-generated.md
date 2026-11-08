## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| locale | string | query | नहीं |  |
| rating | string | query | नहीं |  |
| page | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsTrendingResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getGifsTrending उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let locale = "locale_example" // String |  (वैकल्पिक)
let rating = "rating_example" // String |  (वैकल्पिक)
let page = 987 // Double |  (वैकल्पिक)

PublicAPI.getGifsTrending(tenantId: tenantId, options: PublicAPI.GetGifsTrendingOptions(locale: locale, rating: rating, page: page)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]