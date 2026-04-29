---
모든 에이전트 웹훅은 테넌트의 API 비밀을 사용하여 HMAC-SHA256으로 서명됩니다. 동일한 서명 방식은 FastComments의 댓글 웹훅에도 사용됩니다 - 이미 댓글 웹훅을 통합했다면 에이전트 웹훅은 동일한 서명 헤더와 검증 흐름을 재사용합니다.

### 서명하는 이유

서명이 없으면 웹훅 URL을 아는 공격자가 FastComments에서 보낸 것처럼 보이는 위조된 이벤트를 POST할 수 있습니다. 서명은 엔드포인트가 각 전송이 실제로 유효한지 확인한 후에 처리할 수 있게 해줍니다.

### 서명 작동 방식

각 전송에 대해:

1. 플랫폼은 테넌트 + 일치하는 도메인에 대한 API 비밀을 조회합니다 (자세한 내용은 [웹훅 개요](#webhooks-overview) 참조).
2. 현재 유닉스 타임스탬프(밀리초 단위)를 `X-FastComments-Timestamp` 헤더에 삽입합니다.
3. 플랫폼은 `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe 방식)을 계산하고 결과를 `X-FastComments-Signature` 헤더에 `sha256=<hex>` 형식으로 담아서 전송합니다.
4. 엔드포인트는 타임스탬프 헤더를 읽고, 수신한 `${timestamp}.${body}`에 대해 HMAC를 다시 계산하여 서명 헤더의 `sha256=<hex>` 값과 비교하고 일치하지 않으면 거부합니다.

서명되는 본문은 플랫폼이 전송한 **정확한 바이트**이며 `${timestamp}.`가 접두사로 붙습니다 - 검증기는 재직렬화된 JSON 문자열이 아니라 원시 요청 본문(raw request body)을 사용해야 합니다(그렇지 않으면 키 순서와 공백이 달라집니다).

### API 비밀

댓글 웹훅에서 사용되는 것과 동일한 API 비밀이 사용됩니다 ([댓글 웹훅](/guide-webhooks.html)). 해당 비밀은 (테넌트, 도메인)별로 관리되며 테넌트의 API 설정에서 관리됩니다. 비밀을 교체하는 경우 다음 전송 이전에 새 값을 읽도록 검증기를 재배포해야 합니다.

플랫폼이 일치하는 도메인에 대한 **API 비밀 없음**을 발견하면 전송은 발생하지 않습니다. 웹훅 로그에는 실패 사유로 "no API secret"이 기록됩니다.

### 검증 예시 (Node.js)

[inline-code-attrs-start title = '웹훅 서명 검증 예제'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

`timingSafeEqual`을 `===` 대신 사용하여 서명의 타이밍 채널 누출을 방지하세요.

### 서명된 본문에 포함된 내용

전체 엔벨로프와 이벤트별 `data` 블록이 포함됩니다. 자세한 내용은 [웹훅 페이로드](#webhook-payloads) 참조.

### 권장사항

- **모든 전송에서 검증하세요.** 엔드포인트가 서명되지 않은 요청을 허용하면 무결성 보장이 없습니다.
- **서명 불일치 시 거부하세요.** 401 또는 403을 반환하세요; 잘못된 서명에 대해 200 OK를 반환하면 전송 로그에서 공격이 숨겨집니다.
- **HTTPS를 사용하세요.** 서명은 무결성을 보호하고, TLS는 기밀성을 보호합니다(비밀값과 페이로드의 댓글 텍스트 모두).
- **비밀을 교체하세요** 권한 있는 팀원이 떠나거나 일정에 따라.

### 재플레이 공격 방지

서명만으로는 재플레이 공격을 방지할 수 없습니다 - 실제 서명된 전송을 캡처한 공격자는 이를 재전송할 수 있습니다. 재플레이 방지는 엔드포인트의 책임입니다:

- `occurredAt` 엔벨로프 필드를 사용하고 예를 들어 5분보다 오래된 전송은 거부하세요.
- `triggerId` 또는 `approvalId`를 중복 제거 키(dedup key)로 사용하세요 - 이미 처리한 경우 중복을 무시합니다.

### 관련 항목

- [웹훅 개요](#webhooks-overview).
- [웹훅 페이로드](#webhook-payloads).
- 광범위한 서명 인프라에 대한 주요 [웹훅 가이드](/guide-webhooks.html).
---