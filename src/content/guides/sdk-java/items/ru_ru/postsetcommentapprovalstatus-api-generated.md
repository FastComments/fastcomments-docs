## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| approved | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SetCommentApprovedResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример postSetCommentApprovalStatus'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт классов:
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
    Boolean approved = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      SetCommentApprovedResponse result = apiInstance.postSetCommentApprovalStatus(commentId)
            .approved(approved)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postSetCommentApprovalStatus");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---