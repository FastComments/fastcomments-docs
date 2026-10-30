## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| broadcastId | string | query | Да |  |
| sessionId | string | query | Не |  |
| sso | string | query | Не |  |
| locale | string | query | Не |  |

## Отговор

Връща: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SaveCommentsResponseWithPresence.java)

## Пример

[inline-code-attrs-start title = 'createCommentPublic Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт на класове:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String broadcastId = "broadcastId_example"; // String | 
    CommentData commentData = new CommentData(); // CommentData | 
    String sessionId = "sessionId_example"; // String | 
    String sso = "sso_example"; // String | 
    String locale = "locale_example"; // String | 
    try {
      SaveCommentsResponseWithPresence result = apiInstance.createCommentPublic(tenantId, urlId, broadcastId, commentData)
            .sessionId(sessionId)
            .sso(sso)
            .locale(locale)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Изключение при извикване на PublicApi#createCommentPublic");
      System.err.println("Код на състоянието: " + e.getCode());
      System.err.println("Причина: " + e.getResponseBody());
      System.err.println("Заглавки на отговора: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]