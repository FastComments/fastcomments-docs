댓글에 투표를 첨부하거나, 이미 있는 투표의 전체 상태를 설정합니다.

옵션은 id로 매칭됩니다: 기존 옵션의 id가 포함된 옵션은 투표 수를 유지하며(새 라벨과 위치를 적용), id 없이 전송된 옵션은 추가되고, 목록에서 누락된 기존 옵션은 해당 옵션에 대한 투표와 함께 제거됩니다.

투표가 있는 투표에서 기존 옵션 id를 모두 제외하면 모든 투표가 삭제되므로, 이 경우 replaceVotes=true가 필요합니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | path | 예 |  |
| replaceVotes | boolean | query | 아니요 |  |

## 응답

반환값: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## 예제

[inline-code-attrs-start title = 'putPoll 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    CommentPollPutInput commentPollPutInput = new CommentPollPutInput(); // CommentPollPutInput | 
    Boolean replaceVotes = true; // Boolean | 
    try {
      SavePollResponse result = apiInstance.putPoll(tenantId, commentId, commentPollPutInput)
            .replaceVotes(replaceVotes)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#putPoll");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]