### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате да подесите ваш API кључ на ApiClient пре него што пошаљете аутентификоване захтеве. Ако то не урадите, захтеви ће пропасти са 401 грешком.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Креирајте и конфигуришите API клијента
        ApiClient apiClient = new ApiClient();

        // ОБАВЕЗНО: Поставите ваш API кључ (преузмите га са FastComments контролне табле)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Креирајте API инстанцу са конфигурисаним клијентом
        DefaultApi api = new DefaultApi(apiClient);

        // Сада можете да правите аутентификоване API позиве
        try {
            // Пример: Додавање SSO корисника
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Уобичајене грешке:
            // - 401: API кључ недостаје или није важећи
            // - 400: Валидација захтева није успела
        }
    }
}
```

### Коришћење јавних API-ја (PublicApi)

Јавни ендпоинти не захтевају аутентификацију:

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

### Уобичајени проблеми

1. **401 "missing-api-key" грешка**: Уверите се да позивате `apiClient.setApiKey("YOUR_KEY")` пре него што креирате DefaultApi инстанцу.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве.
3. **Null API key**: SDK ће тихо прескочити аутентификацију ако је API кључ null, што ће довести до 401 грешака.