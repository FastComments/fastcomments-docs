### Kimlik Doğrulamalı API'lerin Kullanımı (DefaultApi)

**Önemli:** Kimlik doğrulamalı istekler yapmadan önce ApiClient üzerinde API anahtarınızı ayarlamanız gerekir. Aksi takdirde istekler 401 hatasıyla başarısız olur.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // API istemcisini oluştur ve yapılandır
        ApiClient apiClient = new ApiClient();

        // GEREKLİ: API anahtarınızı ayarlayın (bunu FastComments panonuzdan alın)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Yapılandırılmış istemci ile API örneğini oluştur
        DefaultApi api = new DefaultApi(apiClient);

        // Artık kimlik doğrulamalı API çağrıları yapabilirsiniz
        try {
            // Örnek: Bir SSO kullanıcısı ekleyin
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

### Genel API'lerin Kullanımı (PublicApi)

Herkese açık uç noktalar kimlik doğrulaması gerektirmez:

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

### Moderasyon API'lerinin Kullanımı (ModerationApi)

`ModerationApi`, moderatör panelini yönetir. Her metot, isteğin yapıldığı SSO kimlik doğrulamalı moderatörü tanımlayan bir `sso` parametresi alır:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Moderasyon bekleyen yorumları listele
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Yaygın Sorunlar

1. **401 "missing-api-key" hatası**: DefaultApi örneğini oluşturmadan önce `apiClient.setApiKey("YOUR_KEY")` çağrısını yaptığınızdan emin olun.
2. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`, istemci tarafı/herkese açık istekler için `PublicApi` kullanın.
3. **Null API anahtarı**: API anahtarı null ise SDK kimlik doğrulamasını sessizce atlar ve bu da 401 hatalarına yol açar.