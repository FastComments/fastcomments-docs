---
설문조사에 투표를 기록하거나 기존 투표를 다른 옵션으로 이동합니다. 투표자는 설문당 최대 하나의 투표만 할 수 있으므로, 동일한 투표자에 대해 이 메서드를 다시 호출하면 투표가 추가되는 것이 아니라 기존 투표가 이동됩니다.

이 동작은 사이트의 설문 설정을 따릅니다. 투표가 로그인한 사용자만 가능하도록 설정된 경우, anonUserId만 포함된 투표는 거부되며, 익명 투표는 설문당 IP당 제한됩니다.

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 응답

반환: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreatePollVoteResponse.java)

## 예제

[inline-code-attrs-start title = 'createPollVote 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 클래스 가져오기:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // API 키 인증 구성: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 다음 줄의 주석을 해제하여 API 키에 접두사를 설정합니다. 예: "Token" (기본값은 null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreatePollVoteBody createPollVoteBody = new CreatePollVoteBody(); // CreatePollVoteBody | 
    try {
      CreatePollVoteResponse result = apiInstance.createPollVote(tenantId, createPollVoteBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createPollVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---