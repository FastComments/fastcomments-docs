### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате подесити ваш API кључ на ApiClient пре слања аутентификованих захтјева. Ако то не урадите, захтјеви ће резултирати 401 грешком.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Креирајте и конфигуришите ApiClient
        ApiClient apiClient = new ApiClient();

        // ОБАВЕЗНО: Подесите ваш API кључ (преузмите га са FastComments контролне табле)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Креирајте инстанцу API-ја са конфигурисаним клијентом
        DefaultApi api = new DefaultApi(apiClient);

        // Сада можете извршавати аутентификоване API позиве
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
            // - 401: API кључ недостаје или је невалидан
            // - 400: Валидација захтјева није успјела
        }
    }
}
```

### Коришћење јавних API-ја (PublicApi)

Јавне крајње тачке не захтјевају аутентификацију:

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

### Коришћење модерацијских API-ја (ModerationApi)

The `ModerationApi` drives the moderator dashboard. Each method accepts an `sso` parameter identifying the SSO-authenticated moderator on whose behalf the request is made:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Листа коментара који чекају модерацију
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Уобичајени проблеми

1. **401 "missing-api-key" error**: Увјерите се да позивате `apiClient.setApiKey("YOUR_KEY")` прије креирања инстанце DefaultApi.
2. **Wrong API class**: Користите `DefaultApi` за аутентификоване захтјеве на страни сервера, `PublicApi` за захтјеве на страни клијента/јавне захтјеве.
3. **Null API key**: SDK ће тихо прескочити аутентификацију ако је API кључ null, што резултира 401 грешкама.