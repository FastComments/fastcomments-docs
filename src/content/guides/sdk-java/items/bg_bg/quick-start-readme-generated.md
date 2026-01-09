### Използване на удостоверени API (DefaultApi)

**Важно:** Трябва да зададете своя API ключ в ApiClient преди да правите удостоверени заявки. Ако не го направите, заявките ще завършат с грешка 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Създайте и конфигурирайте ApiClient
        ApiClient apiClient = new ApiClient();

        // ЗАДЪЛЖИТЕЛНО: Задайте своя API ключ (вземете го от таблото за управление на FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Създайте инстанция на API с конфигурирания клиент
        DefaultApi api = new DefaultApi(apiClient);

        // Сега можете да правите удостоверени API повиквания
        try {
            // Пример: Добавяне на SSO потребител
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Чести грешки:
            // - 401: API ключът липсва или е невалиден
            // - 400: Валидирането на заявката се провали
        }
    }
}
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват удостоверяване:

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

### Чести проблеми

1. **401 "missing-api-key" error**: Уверете се, че извиквате `apiClient.setApiKey("YOUR_KEY")` преди да създадете инстанция на DefaultApi.
2. **Wrong API class**: Използвайте `DefaultApi` за удостоверени заявки от страна на сървъра, `PublicApi` за клиентски/публични заявки.
3. **Null API key**: SDK-то ще пропусне автентикацията без съобщение, ако API ключът е null, което води до грешки 401.