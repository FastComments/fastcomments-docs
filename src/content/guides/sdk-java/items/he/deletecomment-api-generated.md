## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| contextUserId | string | query | לא |  |
| isLive | boolean | query | לא |  |

## תגובה

מחזיר: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteCommentResult.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteComment'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // הגדר הרשאה באמצעות מפתח API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // הסירו את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, למשל "Token" (ברירת מחדל null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    String contextUserId = "contextUserId_example"; // String | 
    Boolean isLive = true; // Boolean | 
    try {
      DeleteCommentResult result = apiInstance.deleteComment(tenantId, id)
            .contextUserId(contextUserId)
            .isLive(isLive)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // חריגה בעת קריאה ל-DefaultApi#deleteComment
      System.err.println("Exception when calling DefaultApi#deleteComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]