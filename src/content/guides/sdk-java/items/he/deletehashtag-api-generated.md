## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tag | string | path | כן |  |
| tenantId | string | query | לא |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/FlagCommentPublic200Response.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteHashTag'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // הגדר הרשאת מפתח ה-API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // הסר את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה "Token" (ברירת המחדל null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tag = "tag_example"; // String | 
    String tenantId = "tenantId_example"; // String | 
    DeleteHashTagRequest deleteHashTagRequest = new DeleteHashTagRequest(); // DeleteHashTagRequest | 
    try {
      FlagCommentPublic200Response result = apiInstance.deleteHashTag(tag)
            .tenantId(tenantId)
            .deleteHashTagRequest(deleteHashTagRequest)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteHashTag");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]