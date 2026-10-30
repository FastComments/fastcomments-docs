הקולות האישיים מאחורי ספירת הצבעות של סקר אחד, מהישנים ביותר לחדשים.

סקר שייך לתגובה, ולכן הקולות תמיד נקראים סקר אחד בכל פעם - נדרש **commentId**. זה שומר שכל שאילתה תתבצע על האינדקסים שכבר קיימים באוסף.

מציית לפרטיות של הסקר: קולות של סקר אנונימי אינם ניתנים לקריאה (**poll-anonymous**), כאן או לפי מזהה.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## תגובה

מחזיר: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVotesResponse.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPollVotes'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // בטל את ההערה על השורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה "Token" (ברירת מחדל היא null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String voterId = "voterId_example"; // String | 
    String optionId = "optionId_example"; // String | 
    Integer skip = 56; // Integer | 
    try {
      GetPollVotesResponse result = apiInstance.getPollVotes(tenantId, commentId)
            .voterId(voterId)
            .optionId(optionId)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPollVotes");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]