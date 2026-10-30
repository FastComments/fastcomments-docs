Недавние комментарии в точном формате, используемом при доставке веб‑хуков, для создания интеграций (например, образцы данных Zapier). Каждое событие передаёт один и тот же объект комментария, поэтому параметр `event` должен быть действительным.

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| event | string | query | No |  |
| limit | number | query | No |  |

## Ответ

Возвращает: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetWebhookSamplePayloadsResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример getWebhookSamplePayloads'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортировать классы:
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
    
    // Настроить авторизацию ключом API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Раскомментировать следующую строку, чтобы установить префикс для ключа API, например "Token" (по умолчанию null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    WebhookEventName event = WebhookEventName.fromValue("comment-created"); // WebhookEventName | 
    Double limit = 3.4D; // Double | 
    try {
      GetWebhookSamplePayloadsResponse result = apiInstance.getWebhookSamplePayloads(tenantId)
            .event(event)
            .limit(limit)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getWebhookSamplePayloads");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]