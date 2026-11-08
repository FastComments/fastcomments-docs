किसी tenant के लिए pages की सूची लाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी room list भरने के लिए उपयोग किया जाता है।
प्रत्येक page के resolved custom config पर `enableFChat` true होना आवश्यक है।
जो pages SSO की मांग करते हैं, उन्हें अनुरोध करने वाले user के group access के विरुद्ध फ़िल्टर किया जाता है।

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| cursor | string | query | नहीं | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी pagination cursor। उसी `sortBy` से बंधा होता है। |
| limit | integer | query | नहीं | 1..200, डिफ़ॉल्ट 50 |
| q | string | query | नहीं | वैकल्पिक, केस-असंवेदनशील title prefix फ़िल्टर। |
| sortBy | string | query | नहीं | क्रमबद्ध क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सबसे अधिक comments पहले), या `title` (वर्णानुक्रम)। |
| hasComments | boolean | query | नहीं | यदि true, तो केवल कम से कम एक comment वाले pages लौटाएं। |

## Response

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Example

[inline-code-attrs-start title = 'getPagesPublic उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String cursor = "cursor_example"; // String | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी pagination cursor। उसी `sortBy` से बंधा होता है।
    Integer limit = 56; // Integer | 1..200, डिफ़ॉल्ट 50
    String q = "q_example"; // String | वैकल्पिक, केस-असंवेदनशील title prefix फ़िल्टर।
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | क्रमबद्ध क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सबसे अधिक comments पहले), या `title` (वर्णानुक्रम)।
    Boolean hasComments = true; // Boolean | यदि true, तो केवल कम से कम एक comment वाले pages लौटाएं।
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]