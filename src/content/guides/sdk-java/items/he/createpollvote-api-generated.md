רשום הצבעה בסקר, או העבר הצבעה קיימת לאופציה אחרת. לכל מצביע יש הצבעה אחת לכל סקר, ולכן קריאה חוזרת עבור אותו מצביע תזיז את הצביעתו במקום להוסיף הצבעה חדשה.

זה מציית להגדרות הסקר של האתר: אם ההצבעה מוגדרת למשתמשים מחוברים בלבד, הצבעה עם רק anonUserId תידחה, והצבעות אנונימיות מוגבלות בקצב לפי IP לכל סקר.

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## Response

מחזיר: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreatePollVoteResponse.java)

## Example

[inline-code-attrs-start title = 'דוגמת createPollVote'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // בטל את ההערה של השורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה "Token" (ברירת מחדל היא null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreatePollVoteBody createPollVoteBody = new CreatePollVoteBody(); // CreatePollVoteBody | 
    try {
      CreatePollVoteResponse result = apiInstance.createPollVote(tenantId, createPollVoteBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // חריגה בעת קריאה ל-DefaultApi#createPollVote
      System.err.println("Exception when calling DefaultApi#createPollVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]