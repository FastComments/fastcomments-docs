### 인증된 API 사용 (DefaultApi)

**Important:** 인증된 요청을 하기 전에 ApiClient에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류로 실패합니다.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // API 클라이언트를 생성하고 구성합니다
        ApiClient apiClient = new ApiClient();

        // 필수: API 키를 설정하세요 (FastComments 대시보드에서 가져옵니다)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 구성된 클라이언트로 API 인스턴스를 만듭니다
        DefaultApi api = new DefaultApi(apiClient);

        // 이제 인증된 API 호출을 할 수 있습니다
        try {
            // 예시: SSO 사용자 추가
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // 일반적인 오류:
            // - 401: API 키가 없거나 유효하지 않습니다
            // - 400: 요청 검증에 실패했습니다
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

### 모더레이션 API 사용 (ModerationApi)

The `ModerationApi`는 모더레이터 대시보드를 구동합니다. 각 메서드는 요청이 대리로 수행되는 SSO 인증된 모더레이터를 식별하는 `sso` 파라미터를 받습니다:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // 모더레이션 대기 중인 댓글 목록을 가져옵니다
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### 일반적인 문제

1. **401 "missing-api-key" 오류**: `DefaultApi` 인스턴스를 생성하기 전에 `apiClient.setApiKey("YOUR_KEY")`를 호출했는지 확인하세요.
2. **잘못된 API 클래스**: 서버 쪽 인증된 요청에는 `DefaultApi`를 사용하고, 클라이언트/공개 요청에는 `PublicApi`를 사용하세요.
3. **API 키가 null인 경우**: SDK는 API 키가 null이면 인증을 조용히 건너뛰어 401 오류를 발생시킵니다.