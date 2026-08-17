## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Одговор

Враћа: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationAPIChildCommentsResponse.java)

## Пример

[inline-code-attrs-start title = 'getCommentChildren Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увези класе:
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
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationAPIChildCommentsResponse result = apiInstance.getCommentChildren(tenantId, commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Изузетак при позиву ModerationApi#getCommentChildren");
      System.err.println("Статусни код: " + e.getCode());
      System.err.println("Разлог: " + e.getResponseBody());
      System.err.println("Заглавља одговора: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]