### Використання автентифікованих API (DefaultApi)

**Важливо:** Ви повинні встановити свій API ключ у ApiClient перед виконанням автентифікованих запитів. Якщо ви цього не зробите, запити завершаться помилкою 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Створіть та налаштуйте клієнт API
        ApiClient apiClient = new ApiClient();

        // ОБОВ'ЯЗКОВО: Встановіть свій API-ключ (отримуйте його в панелі керування FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Створіть екземпляр API з налаштованим клієнтом
        DefaultApi api = new DefaultApi(apiClient);

        // Тепер ви можете виконувати автентифіковані виклики API
        try {
            // Приклад: Додати SSO-користувача
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Типові помилки:
            // - 401: API-ключ відсутній або недійсний
            // - 400: Помилка валідації запиту
        }
    }
}
```

### Використання публічних API (PublicApi)

Публічні кінцеві точки не потребують автентифікації:

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

### Використання модераційних API (ModerationApi)

Клас `ModerationApi` керує панеллю модератора. Кожен метод приймає параметр `sso`, який ідентифікує модератора, автентифікованого через SSO, від імені якого виконується запит:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Перелік коментарів, що очікують модерації
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Типові проблеми

1. **401 "missing-api-key" помилка**: Переконайтеся, що ви викликаєте `apiClient.setApiKey("YOUR_KEY")` перед створенням екземпляра DefaultApi.
2. **Неправильний клас API**: Використовуйте `DefaultApi` для серверних автентифікованих запитів, `PublicApi` для клієнтських/публічних запитів.
3. **Null API key**: SDK мовчки пропустить автентифікацію, якщо API-ключ дорівнює null, що призведе до помилок 401.