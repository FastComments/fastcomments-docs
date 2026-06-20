## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Так |  |
| spam | boolean | query | Ні |  |
| permNotSpam | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Приклад

[inline-code-attrs-start title = 'Приклад postSetCommentSpamStatus'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Імпорт класів:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String commentId = "commentId_example"; // String | 
    Boolean spam = true; // Boolean | 
    Boolean permNotSpam = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.postSetCommentSpamStatus(commentId)
            .spam(spam)
            .permNotSpam(permNotSpam)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postSetCommentSpamStatus");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]