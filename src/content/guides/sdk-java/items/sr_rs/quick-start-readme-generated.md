### Коришћење аутентификованих API-ја (DefaultApi)

**Important:** You must set your API key on the ApiClient before making authenticated requests. If you don't, requests will fail with a 401 error.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Креирајте и конфигуришите ApiClient
        ApiClient apiClient = new ApiClient();

        // ОБАВЕЗНО: Подесите ваш API кључ (преузмите га са вашe FastComments контролне табле)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Креирајте инстанцу API-ја са конфигурисаним клијентом
        DefaultApi api = new DefaultApi(apiClient);

        // Сада можете да правите аутентификоване API захтеве
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
            // - 401: API кључ недостаје или је неважећи
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

### Коришћење модерацијских API-ја (ModerationApi)

Класа `ModerationApi` покреће контролну таблу модератора. Сва метода прихвата параметар `sso` који идентификује SSO-аутентификованог модератора у чије име се захтев обавља:

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

1. **401 "missing-api-key" error**: Уверите се да позивате `apiClient.setApiKey("YOUR_KEY")` пре него што креирате инстанцу DefaultApi.
2. **Погрешна класа API-ја**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве.
3. **Null API key**: SDK ће тихо прескочити аутентификацију ако је API кључ null, што ће довести до 401 грешака.