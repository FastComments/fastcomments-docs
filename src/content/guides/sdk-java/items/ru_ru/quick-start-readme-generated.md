### Using Authenticated APIs (DefaultApi)

**Важно:** Вы должны установить ваш API-ключ в ApiClient перед выполнением аутентифицированных запросов. Если вы этого не сделаете, запросы завершатся с ошибкой 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Создаём и настраиваем API-клиент
        ApiClient apiClient = new ApiClient();

        // REQUIRED: Set your API key (get this from your FastComments dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Create the API instance with the configured client
        DefaultApi api = new DefaultApi(apiClient);

        // Now you can make authenticated API calls
        try {
            // Example: Add an SSO user
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Распространённые ошибки:
            // - 401: API-ключ отсутствует или недействителен
            // - 400: Ошибка валидации запроса
        }
    }
}
```

### Using Public APIs (PublicApi)

Публичные эндпойнты не требуют аутентификации:

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

### Using Moderation APIs (ModerationApi)

The `ModerationApi` drives the moderator dashboard. Each method accepts an `sso` parameter identifying the SSO-authenticated moderator on whose behalf the request is made:

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

### Common Issues

1. **401 "missing-api-key" error**: Убедитесь, что вы вызываете `apiClient.setApiKey("YOUR_KEY")` перед созданием экземпляра DefaultApi.
2. **Wrong API class**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов.
3. **Null API key**: SDK молча пропустит аутентификацию, если API-ключ равен null, что приведёт к ошибкам 401.