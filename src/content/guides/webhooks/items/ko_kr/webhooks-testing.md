Webhooks admin에는 각 이벤트 유형(Create, Update, Delete)마다 `Send Test Payload` 버튼이 있습니다. Create 및 Update 이벤트는 더미 WebhookComment 객체를 전송하는 반면, Delete 테스트는 ID만 포함된 더미 요청 본문을 전송합니다.

테스트는 "happy" (correct API Key) 및 "sad" (invalid API key) 시나리오에 대한 응답 코드를 확인하기 위해 두 번 호출을 수행합니다.

테스트가 invalid API key를 보낼 때 테스트가 완전히 통과하려면 상태 코드 401을 반환해야 합니다. token 값을 올바르게 확인하지 않으면 오류가 발생합니다.

이는 요청을 올바르게 인증하는지 확인하기 위한 것입니다.