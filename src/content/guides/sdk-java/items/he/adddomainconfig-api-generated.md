## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`AddDomainConfig200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AddDomainConfig200Response.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-addDomainConfig'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // הגדרת הרשאת מפתח API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // הסר את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, למשל "Token" (ברירת מחדל null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    AddDomainConfigParams addDomainConfigParams = new AddDomainConfigParams(); // AddDomainConfigParams | 
    try {
      AddDomainConfig200Response result = apiInstance.addDomainConfig(tenantId, addDomainConfigParams)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#addDomainConfig");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]