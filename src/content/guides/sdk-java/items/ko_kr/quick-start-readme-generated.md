### 인증된 API 사용 (DefaultApi)

**중요:** 인증된 요청을 수행하기 전에 ApiClient에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류로 실패합니다.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // API 클라이언트를 생성하고 구성합니다
        ApiClient apiClient = new ApiClient();

        // REQUIRED: Set your API key (get this from your FastComments dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 구성된 클라이언트로 API 인스턴스를 생성합니다
        DefaultApi api = new DefaultApi(apiClient);

        // 이제 인증된 API 호출을 할 수 있습니다
        try {
            // Example: Add an SSO user
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Common errors:
            // - 401: API key is missing or invalid
            // - 400: Request validation failed
        }
    }
}
```

### 공개 API 사용 (PublicApi)

공개 엔드포인트는 인증이 필요하지 않습니다:

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

### 자주 발생하는 문제

1. **401 "missing-api-key" 오류**: `apiClient.setApiKey("YOUR_KEY")`를 DefaultApi 인스턴스를 생성하기 전에 호출했는지 확인하세요.
2. **잘못된 API 클래스**: 서버 측 인증 요청에는 `DefaultApi`를, 클라이언트 측/공개 요청에는 `PublicApi`를 사용하세요.
3. **API 키가 null인 경우**: SDK는 API 키가 null이면 인증을 조용히 건너뛰어 401 오류가 발생합니다.