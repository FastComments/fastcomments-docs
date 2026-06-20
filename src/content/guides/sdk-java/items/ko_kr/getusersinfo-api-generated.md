---
테넌트에 대한 대량 사용자 정보입니다. userIds가 주어지면 User / SSOUser의 표시 정보를 반환합니다.
댓글 위젯에서 presence 이벤트를 통해 방금 나타난 사용자를 보강하기 위해 사용됩니다.
페이지 컨텍스트 없음: 개인정보 보호는 일관되게 적용됩니다(비공개 프로필은 마스킹됩니다).

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| ids | string | query | 예 | 쉼표로 구분된 userIds. |

## 응답

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersInfoResponse.java)

## 예제

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 클래스 가져오기:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // 문자열 | 
    String ids = "ids_example"; // 문자열 | 쉼표로 구분된 userIds.
    try {
      PageUsersInfoResponse result = apiInstance.getUsersInfo(tenantId, ids)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUsersInfo");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---