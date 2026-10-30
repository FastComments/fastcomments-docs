Запишете глас в анкета или преместете съществуващ глас към различна опция. Гласуващият може да има най‑много един глас за анкета, така че повторното извикване за същия гласуващ премества гласа му, вместо да добавя нов.

Това спазва настройките на анкетата в сайта: ако гласуването е разрешено само за вписани потребители, глас с само `anonUserId` се отказва, а анонимните гласове са ограничени по брой за IP за всяка анкета.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreatePollVoteResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за createPollVote'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт на класове:
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
    
    // Конфигуриране на упълномощаване с API ключ: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Разкоментирайте следния ред, за да зададете префикс за API ключа, напр. "Token" (по подразбиране null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreatePollVoteBody createPollVoteBody = new CreatePollVoteBody(); // CreatePollVoteBody | 
    try {
      CreatePollVoteResponse result = apiInstance.createPollVote(tenantId, createPollVoteBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Изключение при извикване на DefaultApi#createPollVote
      System.err.println("Exception when calling DefaultApi#createPollVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]