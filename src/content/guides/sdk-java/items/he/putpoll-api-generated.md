צירוף סקר לתגובה, או הגדרת המצב המלא של הסקר שכבר קיים לה.

האפשרויות מותאמות לפי id: אפשרות שנשלחת עם ה-id של אפשרות קיימת שומרת על ההצבעות שלה (ומקבלת את
התווית והמיקום החדשים), אפשרות שנשלחת ללא id מתווספת, ואפשרויות קיימות שלא נכללו ברשימה מוסרות יחד עם
ההצבעות שניתנו להן.

אם לא נשמר אף id של אפשרות קיימת בסקר שיש בו הצבעות, כל ההצבעות יימחקו, ולכן נדרש replaceVotes=true.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| replaceVotes | boolean | query | No |  |

## תגובה

מחזיר: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-putPoll'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    CommentPollPutInput commentPollPutInput = new CommentPollPutInput(); // CommentPollPutInput | 
    Boolean replaceVotes = true; // Boolean | 
    try {
      SavePollResponse result = apiInstance.putPoll(tenantId, commentId, commentPollPutInput)
            .replaceVotes(replaceVotes)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#putPoll");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]