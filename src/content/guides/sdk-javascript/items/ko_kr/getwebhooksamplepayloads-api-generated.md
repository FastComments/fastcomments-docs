웹훅 전송에 사용되는 정확한 형태의 최근 댓글로, 통합 기능을 구축하기 위한 것입니다(예: Zapier 샘플 데이터). 모든 이벤트는 동일한 댓글 객체를 전달하므로 `event`만 유효하면 됩니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| event | WebhookEventName | 아니요 |  |
| limit | number | 아니요 |  |

## 응답

반환값: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getWebhookSamplePayloads 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// 특정 이벤트 타입에 대한 샘플 페이로드 가져오기
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// 모든 샘플 페이로드 가져오기 (선택적 매개변수 없음)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]