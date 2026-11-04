### Использование аутентифицированных API (DefaultApi)

**Важно:** Перед выполнением аутентифицированных запросов необходимо установить ваш API-ключ в ApiClient. Если этого не сделать, запросы завершатся ошибкой 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Создание и настройка API-клиента
        ApiClient apiClient = new ApiClient();

        // ОБЯЗАТЕЛЬНО: Установите ваш API-ключ (получите его в панели управления FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Создание экземпляра API с настроенным клиентом
        DefaultApi api = new DefaultApi(apiClient);

        // Теперь можно выполнять аутентифицированные вызовы API
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
            // Частые ошибки:
            // - 401: API-ключ отсутствует или недействителен
            // - 400: Не прошла валидация запроса
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

### Использование API модерации (ModerationApi)

`ModerationApi` управляет панелью модератора. Каждый метод принимает параметр `sso`, идентифицирующий SSO-аутентифицированного модератора, от имени которого выполняется запрос:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Список комментариев, ожидающих модерации
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Частые проблемы

1. **Ошибка 401 "missing-api-key"**: Убедитесь, что вы вызываете `apiClient.setApiKey("YOUR_KEY")` перед созданием экземпляра DefaultApi.
2. **Неправильный класс API**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` — для клиентских/публичных запросов.
3. **Null API-ключ**: SDK молча пропустит аутентификацию, если API-ключ равен null, что приведёт к ошибкам 401.