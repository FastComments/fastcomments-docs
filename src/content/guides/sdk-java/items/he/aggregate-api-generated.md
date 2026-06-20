מאגד מסמכים על ידי קיבוצם (אם מסופק groupBy) והפעלת מספר פעולות.
נתמכות פעולות שונות (למשל sum, countDistinct, avg וכו').

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| parentTenantId | string | שאילתה | לא |  |
| includeStats | boolean | שאילתה | לא |  |

## תגובה

מחזיר: [`AggregateResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AggregateResponse.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-aggregate'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // הגדר הרשאת מפתח API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // בטל את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה "Token" (ברירת מחדל: null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    AggregationRequest aggregationRequest = new AggregationRequest(); // AggregationRequest | 
    String parentTenantId = "parentTenantId_example"; // String | 
    Boolean includeStats = true; // Boolean | 
    try {
      AggregateResponse result = apiInstance.aggregate(tenantId, aggregationRequest)
            .parentTenantId(parentTenantId)
            .includeStats(includeStats)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#aggregate");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]