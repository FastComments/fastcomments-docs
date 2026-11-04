사람의 가입 절차 없이 AI 에이전트를 위한 새 체험 계정을 생성합니다. 이 호출에는 API 키가 필요하지 않습니다.

응답에는 테넌트 ID, REST API 및 MCP 서버에 즉시 사용할 수 있는 API 키, 그리고 클레임 URL이 포함됩니다. 클레임 URL은 작업 대상인 사람에게 전달하세요: FastComments에 로그인한 상태에서 해당 URL을 열면 계정이 그 사람에게 연결됩니다. 클레임되지 않은 계정과 해당 키는 생성 후 72시간이 지나면 삭제됩니다. 클레임되기 전까지 계정에는 표준 체험 제한이 적용됩니다.

## 응답

반환값: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## 예제

[inline-code-attrs-start title = 'createAgentTenant 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]