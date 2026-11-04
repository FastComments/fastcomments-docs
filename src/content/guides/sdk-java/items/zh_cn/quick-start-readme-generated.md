### 使用经过身份验证的 API（DefaultApi）

**重要提示：** 在发起经过身份验证的请求之前，您必须在 ApiClient 上设置您的 API 密钥。否则，请求将失败并返回 401 错误。

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // 创建并配置 API 客户端
        ApiClient apiClient = new ApiClient();

        // 必需：设置您的 API 密钥（从 FastComments 控制面板获取）
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 使用配置好的客户端创建 API 实例
        DefaultApi api = new DefaultApi(apiClient);

        // 现在您可以发起经过身份验证的 API 调用
        try {
            // 示例：添加一个 SSO 用户
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // 常见错误：
            // - 401：API 密钥缺失或无效
            // - 400：请求验证失败
        }
    }
}
```

### 使用公共 API（PublicApi）

公共端点不需要身份验证：

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

### 使用审核 API（ModerationApi）

`ModerationApi` 用于驱动审核员控制面板。每个方法都接受一个 `sso` 参数，用于标识代表其发起请求的经过 SSO 身份验证的审核员：

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // 列出等待审核的评论
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### 常见问题

1. **401 "missing-api-key" 错误**：请确保在创建 DefaultApi 实例之前调用 `apiClient.setApiKey("YOUR_KEY")`。
2. **API 类使用错误**：服务端经过身份验证的请求请使用 `DefaultApi`，客户端/公共请求请使用 `PublicApi`。
3. **API 密钥为空**：如果 API 密钥为 null，SDK 将静默跳过身份验证，从而导致 401 错误。