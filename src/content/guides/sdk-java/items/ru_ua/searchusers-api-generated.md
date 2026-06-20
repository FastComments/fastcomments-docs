## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| usernameStartsWith | string | query | Нет |  |
| mentionGroupIds | array | query | Нет |  |
| sso | string | query | Нет |  |
| searchSection | string | query | Нет |  |

## Ответ

Возвращает: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SearchUsersResult.java)

## Пример

[inline-code-attrs-start title = 'Пример searchUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт классов:
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
    String usernameStartsWith = "usernameStartsWith_example"; // String | 
    List<String> mentionGroupIds = Arrays.asList(); // List<String> | 
    String sso = "sso_example"; // String | 
    String searchSection = "fast"; // String | 
    try {
      SearchUsersResult result = apiInstance.searchUsers(tenantId, urlId)
            .usernameStartsWith(usernameStartsWith)
            .mentionGroupIds(mentionGroupIds)
            .sso(sso)
            .searchSection(searchSection)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#searchUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]