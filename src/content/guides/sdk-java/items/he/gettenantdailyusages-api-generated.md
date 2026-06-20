## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| yearNumber | number | query | לא |  |
| monthNumber | number | query | לא |  |
| dayNumber | number | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTenantDailyUsagesResponse.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenantDailyUsages'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// יבוא מחלקות:
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
    
    // הגדרת אישור מפתח API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // בטל את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, למשל "Token" (ברירת מחדל null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Double yearNumber = 3.4D; // Double | 
    Double monthNumber = 3.4D; // Double | 
    Double dayNumber = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    try {
      GetTenantDailyUsagesResponse result = apiInstance.getTenantDailyUsages(tenantId)
            .yearNumber(yearNumber)
            .monthNumber(monthNumber)
            .dayNumber(dayNumber)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getTenantDailyUsages");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]