### Kimlik Doğrulamalı API'leri Kullanma (DefaultApi)

**Önemli:** Kimlik doğrulamalı istekler yapmadan önce ApiClient üzerinde API anahtarınızı ayarlamanız gerekir. Ayarlamazsanız, istekler 401 hatası ile başarısız olur.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // API istemcisini oluşturun ve yapılandırın
        ApiClient apiClient = new ApiClient();

        // ZORUNLU: API anahtarınızı ayarlayın (bunu FastComments kontrol panelinizden alın)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Yapılandırılmış istemci ile API örneğini oluşturun
        DefaultApi api = new DefaultApi(apiClient);

        // Artık kimlik doğrulamalı API çağrıları yapabilirsiniz
        try {
            // Örnek: Bir SSO kullanıcısı ekle
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Yaygın hatalar:
            // - 401: API anahtarı eksik veya geçersiz
            // - 400: İstek doğrulaması başarısız oldu
        }
    }
}
```

### Public API'leri Kullanma (PublicApi)

Genel uç noktalar kimlik doğrulama gerektirmez:

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

### Moderasyon API'lerini Kullanma (ModerationApi)

The `ModerationApi` drives the moderator dashboard. Each method accepts an `sso` parameter identifying the SSO-authenticated moderator on whose behalf the request is made:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Moderasyona alınmayı bekleyen yorumları listele
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Yaygın Sorunlar

1. **401 "missing-api-key" error**: DefaultApi örneğini oluşturmadan önce `apiClient.setApiKey("YOUR_KEY")` çağırdığınızdan emin olun.
2. **Wrong API class**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`'yi, istemci tarafı/genel istekler için `PublicApi`'yi kullanın.
3. **Null API key**: API anahtarı null ise SDK kimlik doğrulamayı sessizce atlar ve bu da 401 hatalarına yol açar.