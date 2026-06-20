## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetUserInternalProfileResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за getUserInternalProfile'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортиране на класове:
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
    String sso = "sso_example"; // String | 
    try {
      GetUserInternalProfileResponse result = apiInstance.getUserInternalProfile()
            .commentId(commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getUserInternalProfile");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---