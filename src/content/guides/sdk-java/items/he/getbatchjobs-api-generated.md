The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| type | string | query | לא | רק משימות מסוג זה, לדוגמה migrate-pages. |
| status | string | query | לא | רק משימות במצב זה: requested, running, success או failed. |
| skip | integer | query | לא | כמה משימות לדלג, עבור דפדוף. |

## Response

Returns: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBatchJobsAPIResponse.java)

## Example

[inline-code-attrs-start title = 'דוגמת getBatchJobs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// ייבא מחלקות:
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
    // בטל את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה "Token" (ברירת מחדל null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String type = "type_example"; // String | רק משימות מסוג זה, לדוגמה migrate-pages.
    String status = "status_example"; // String | רק משימות במצב זה: requested, running, success או failed.
    Integer skip = 56; // Integer | כמה משימות לדלג, עבור דפדוף.
    try {
      GetBatchJobsAPIResponse result = apiInstance.getBatchJobs(tenantId)
            .type(type)
            .status(status)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getBatchJobs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]