### Kimlik Doğrulamalı API'leri Kullanma (DefaultApi)

**Önemli:** Kimlik doğrulamalı istekler yapmadan önce ApiClient üzerinde API anahtarınızı ayarlamalısınız. Eğer yapmazsanız, istekler 401 hatası ile başarısız olur.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // API istemcisini oluşturun ve yapılandırın
        ApiClient apiClient = new ApiClient();

        // GEREKLİ: API anahtarınızı ayarlayın (bunu FastComments kontrol panelinizden alın)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Yapılandırılmış istemci ile API örneğini oluşturun
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

### Public API'leri Kullanma (PublicApi)

Herkese açık uç noktalar kimlik doğrulama gerektirmez:

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

### Yaygın Sorunlar

1. **401 "missing-api-key" hatası**: DefaultApi örneğini oluşturmadan önce `apiClient.setApiKey("YOUR_KEY")` çağırdığınızdan emin olun.
2. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`'yi, istemci tarafı/genel istekler için `PublicApi`'yi kullanın.
3. **Null API anahtarı**: API anahtarı null ise SDK kimlik doğrulamasını sessizce atlayacaktır; bu da 401 hatalarına yol açar.