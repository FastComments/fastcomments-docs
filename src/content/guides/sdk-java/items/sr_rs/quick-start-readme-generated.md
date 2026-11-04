### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате поставити свој API кључ на ApiClient пре слања аутентификованих захтева. Ако то не урадите, захтеви ће пасти са 401 грешком.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Креирање и конфигурисање API клијента
        ApiClient apiClient = new ApiClient();

        // ОБАВЕЗНО: Поставите свој API кључ (добавите га из FastComments контролне табле)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Креирање API инстанце са конфигурисаним клијентом
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
            // - 400: Валидација захтева није успела
        }
    }
}
```

### Коришћење јавних API-ја (PublicApi)

Јавни крајњи тачке не захтевају аутентификацију:

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

### Коришћење API-ја за модерацију (ModerationApi)

`ModerationApi` покреће контролну таблу модератора. Сваки метод прихвата `sso` параметар који идентификује SSO-аутентификованог модератора у чије име се захтев упућује:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Листање коментара који чекају модерацију
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Уобичајени проблеми

1. **401 "missing-api-key" грешка**: Уверите се да позивате `apiClient.setApiKey("YOUR_KEY")` пре креирања DefaultApi инстанце.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, а `PublicApi` за клијентске/јавне захтеве.
3. **Null API кључ**: SDK ће тихо прескочити аутентификацију ако је API кључ null, што доводи до 401 грешака.