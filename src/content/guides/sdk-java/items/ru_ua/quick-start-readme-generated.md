### Использование аутентифицированных API (DefaultApi)

**Важно:** Вы должны установить ваш API-ключ в ApiClient перед выполнением аутентифицированных запросов. Если вы этого не сделаете, запросы завершатся ошибкой 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Создайте и настройте ApiClient
        ApiClient apiClient = new ApiClient();

        // ОБЯЗАТЕЛЬНО: Установите ваш API-ключ (получите его на панели управления FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Создайте экземпляр API с настроенным клиентом
        DefaultApi api = new DefaultApi(apiClient);

        // Теперь вы можете выполнять аутентифицированные вызовы API
        try {
            // Пример: Добавление SSO-пользователя
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Распространенные ошибки:
            // - 401: API-ключ отсутствует или недействителен
            // - 400: Ошибка валидации запроса
        }
    }
}
```

### Использование публичных API (PublicApi)

Публичные эндпоинты не требуют аутентификации:

```java
import com.fastcomments.api.PublicApi;
import com.fastcomments.invoker.ApiException;

PublicApi publicApi = new PublicApi();

try {
    var response = publicApi.getCommentsPublic("YOUR_TENANT_ID", "page-url-id")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Распространенные проблемы

1. **401 "missing-api-key" error**: Убедитесь, что вы вызываете `apiClient.setApiKey("YOUR_KEY")` перед созданием экземпляра DefaultApi.
2. **Wrong API class**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов.
3. **Null API key**: SDK тихо пропустит аутентификацию, если API-ключ равен null, что приведет к ошибкам 401.