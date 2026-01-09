## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetDomainConfigs200Response.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getDomainConfigs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// ייבוא מחלקות:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // קבע הרשאת מפתח API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // בטל את ההערה מהשורה הבאה כדי להגדיר קידומת עבור מפתח ה-API, לדוגמה "Token" (ברירת מחדל null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    try {
      GetDomainConfigs200Response result = apiInstance.getDomainConfigs(tenantId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getDomainConfigs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]