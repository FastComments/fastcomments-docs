## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| page | integer | query | Не |  |
| limit | integer | query | Не |  |
| skip | integer | query | Не |  |
| asTree | boolean | query | Не |  |
| skipChildren | integer | query | Не |  |
| limitChildren | integer | query | Не |  |
| maxTreeDepth | integer | query | Не |  |
| urlId | string | query | Не |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |
| contextUserId | string | query | Не |  |
| hashTag | string | query | Не |  |
| parentId | string | query | Не |  |
| direction | string | query | Не |  |

## Отговор

Връща: [`GetComments200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetComments200Response.java)

## Пример

[inline-code-attrs-start title = 'Пример за getComments'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортирайте класовете:
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
    
    // Конфигуриране на авторизация с API ключ: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Разкоментирайте следния ред, за да зададете префикс за API ключа, напр. "Token" (по подразбиране null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Integer page = 56; // Integer | 
    Integer limit = 56; // Integer | 
    Integer skip = 56; // Integer | 
    Boolean asTree = true; // Boolean | 
    Integer skipChildren = 56; // Integer | 
    Integer limitChildren = 56; // Integer | 
    Integer maxTreeDepth = 56; // Integer | 
    String urlId = "urlId_example"; // String | 
    String userId = "userId_example"; // String | 
    String anonUserId = "anonUserId_example"; // String | 
    String contextUserId = "contextUserId_example"; // String | 
    String hashTag = "hashTag_example"; // String | 
    String parentId = "parentId_example"; // String | 
    SortDirections direction = SortDirections.fromValue("OF"); // SortDirections | 
    try {
      GetComments200Response result = apiInstance.getComments(tenantId)
            .page(page)
            .limit(limit)
            .skip(skip)
            .asTree(asTree)
            .skipChildren(skipChildren)
            .limitChildren(limitChildren)
            .maxTreeDepth(maxTreeDepth)
            .urlId(urlId)
            .userId(userId)
            .anonUserId(anonUserId)
            .contextUserId(contextUserId)
            .hashTag(hashTag)
            .parentId(parentId)
            .direction(direction)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getComments");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]