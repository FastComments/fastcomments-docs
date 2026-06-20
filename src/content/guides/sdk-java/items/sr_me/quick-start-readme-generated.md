### Коришћење аутентификованих API-ја (DefaultApi)

**Важнo:** Морате да подесите ваш API кључ на ApiClient пре слања аутентификованих захтјева. Ако то не урадите, захтјеви ће пропасти са грешком 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Направите и конфигуришите ApiClient
        ApiClient apiClient = new ApiClient();

        // ОБАВЕЗНО: Поставите ваш API кључ (добијете га са FastComments контролне табле)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Направите инстанцу API-ја са конфигурисаним ApiClient-ом
        DefaultApi api = new DefaultApi(apiClient);

        // Сада можете упућивати аутентификоване API позиве
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
            // - 400: Валидација захтјева није успела
        }
    }
}
```

### Коришћење јавних API-ја (PublicApi)

Јавни endpoint-и не захтијевају аутентификацију:

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

`ModerationApi` покреће модераторску контролну таблу. Свaka метода прихвата параметар `sso` који идентификује SSO-аутентификованог модератора у чије име се захтјев прави:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Прикажите коментаре који чекају на модерацију
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Уобичајени проблеми

1. **401 "missing-api-key" error**: Проверите да ли позивате `apiClient.setApiKey("YOUR_KEY")` прије креирања инстанце DefaultApi.
2. **Wrong API class**: Користите `DefaultApi` за аутентификоване захтјеве на серверској страни, `PublicApi` за захтјеве на клијентској страни/јавне захтјеве.
3. **Null API key**: SDK ће тихо прескочити аутентификацију ако је API кључ null, што ће довести до грешака 401.