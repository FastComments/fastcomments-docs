### Използване на автентифицирани API-та (DefaultApi)

**Важно:** Трябва да зададете вашия API ключ в ApiClient преди да правите автентифицирани заявки. Ако не го направите, заявките ще се провалят с грешка 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Създайте и конфигурирайте API клиента
        ApiClient apiClient = new ApiClient();

        // ЗАДЪЛЖИТЕЛНО: Задайте вашия API ключ (вземете го от таблото на FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Създайте екземпляр на API-то с конфигурирания клиент
        DefaultApi api = new DefaultApi(apiClient);

        // Сега можете да правите автентифицирани API извиквания
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
            // Общи грешки:
            // - 401: липсва API ключ или е невалиден
            // - 400: Валидацията на заявката е неуспешна
        }
    }
}
```

### Използване на публични API-та (PublicApi)

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

### Използване на API-та за модерация (ModerationApi)

Класът `ModerationApi` управлява таблото на модератора. Във всеки метод се приема параметър `sso`, който идентифицира SSO-автентифицирания модератор от чие име се прави заявката:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Изброяване на коментари, чакащи модерация
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Чести проблеми

1. **401 "missing-api-key" грешка**: Уверете се, че извиквате `apiClient.setApiKey("YOUR_KEY")` преди да създадете инстанцията на DefaultApi.
2. **Грешен клас API**: Използвайте `DefaultApi` за сървърни автентифицирани заявки, `PublicApi` за клиентски/публични заявки.
3. **Null API ключ**: SDK-то ще пропусне удостоверяването без съобщение, ако API ключът е null, което ще доведе до грешки 401.