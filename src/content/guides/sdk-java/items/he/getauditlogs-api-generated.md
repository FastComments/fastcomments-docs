## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | הדייר לקריאת יומני ביקורת. |
| limit | integer | query | No | כמה יומנים להחזיר. מקסימום 10000, ברירת מחדל 1000. |
| skip | integer | query | No | כמה יומנים לדלג, עבור דפדוף. |
| order | string | query | No | כיוון המיון על זמן האירוע. ברירת מחדל DESC (החדש ביותר ראשון). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | להחזיר רק אירועים שבוצעו על ידי שם משתמש זה. |
| ip | string | query | No | להחזיר רק אירועים מכתובת IP זו. |
| crudType | string | query | No | להחזיר רק אירועים מסוג זה: c, r, u, d, או login. |
| resourceName | string | query | No | להחזיר רק אירועים עבור משאב זה, למשל User או Moderator. |
| targetId | string | query | No | להחזיר רק אירועים שבהם האובייקט המושפע בעל מזהה זה. |
| target | string | query | No | התאמה של תת-מחרוזת ללא תלות ברישיות על תווית האובייקט המושפע, למשל שם משתמש או אימייל. |
| includeManagedTenants | boolean | query | No | גם להחזיר אירועים מדיירים שהדייר הזה מנהל. |

## Response

Returns: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Example

[inline-code-attrs-start title = 'דוגמת getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | הדייר לקריאת יומני ביקורת.
    Integer limit = 56; // Integer | כמה יומנים להחזיר. מקסימום 10000, ברירת מחדל 1000.
    Integer skip = 56; // Integer | כמה יומנים לדלג, עבור דפדוף.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | כיוון המיון על זמן האירוע. ברירת מחדל DESC (החדש ביותר ראשון).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | להחזיר רק אירועים שבוצעו על ידי שם משתמש זה.
    String ip = "ip_example"; // String | להחזיר רק אירועים מכתובת IP זו.
    String crudType = "crudType_example"; // String | להחזיר רק אירועים מסוג זה: c, r, u, d, או login.
    String resourceName = "resourceName_example"; // String | להחזיר רק אירועים עבור משאב זה, למשל User או Moderator.
    String targetId = "targetId_example"; // String | להחזיר רק אירועים שבהם האובייקט המושפע בעל מזהה זה.
    String target = "target_example"; // String | התאמה של תת-מחרוזת ללא תלות ברישיות על תווית האובייקט המושפע, למשל שם משתמש או אימייל.
    Boolean includeManagedTenants = true; // Boolean | גם להחזיר אירועים מדיירים שהדייר הזה מנהל.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]