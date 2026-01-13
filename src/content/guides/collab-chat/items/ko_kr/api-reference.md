### API 개요

Collab Chat은 텍스트 대화를 프로그래밍 방식으로 관리하기 위한 세 개의 REST API 엔드포인트를 제공합니다. 이 엔드포인트를 사용하면 브라우저 위젯을 사용하지 않고도 주석을 검색, 생성 및 삭제할 수 있습니다.

이들은 브라우저 쿠키를 통해 사용자를 인증하는 공개 엔드포인트입니다. API 키를 사용하지 않습니다. 사용자는 이러한 엔드포인트에 액세스하려면 브라우저에서 FastComments에 로그인되어 있어야 합니다.

### 기본 URL

모든 Collab Chat API 엔드포인트는 다음 하위에 있습니다:

[inline-code-attrs-start title = '기본 URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### 인증

이 엔드포인트들은 브라우저 쿠키를 통해 사용자를 인증합니다. API 키를 사용하지 않습니다. 사용자는 이러한 엔드포인트에 액세스하려면 브라우저에서 FastComments에 로그인되어 있어야 합니다.

### 모든 대화 가져오기

특정 페이지에 대한 모든 텍스트 대화를 검색합니다.

#### 엔드포인트

[inline-code-attrs-start title = 'GET 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### 매개변수

`tenantId` (경로 매개변수, 필수)는 귀하의 FastComments Tenant ID입니다.

`urlId` (쿼리 매개변수, 필수)는 대화를 검색하려는 페이지 식별자입니다.

#### 응답

응답에는 API 상태, 인증된 경우 현재 사용자 세션 정보, ID, URL 및 텍스트 범위가 포함된 대화 배열, 정리된 URL 식별자, 현재 사용자가 사이트 관리자 또는 중재자인지 여부를 나타내는 플래그, 그리고 실시간 동기화를 위한 WebSocket 연결 세부정보(`tenantIdWS`, `urlIdWS`, 및 `userIdWS`)가 포함됩니다.

#### 예시 요청

[inline-code-attrs-start title = 'GET 요청 예시'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### 예시 응답

[inline-code-attrs-start title = 'GET 응답 예시'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### 대화 생성

특정 텍스트 선택에 대해 새 텍스트 대화를 생성합니다.

#### 엔드포인트

[inline-code-attrs-start title = 'POST 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### 매개변수

`tenantId` (경로 매개변수, 필수)는 귀하의 FastComments Tenant ID입니다.

요청 본문은 JSON이어야 하며 다음 필수 필드를 포함해야 합니다.

`urlId` (string, 필수)는 기본 페이지 식별자입니다.

`urlIdWithRange` (string, 필수)는 텍스트 범위가 결합된 URL로, 예를 들어 `my-page:p:0:15,0:45{abc123}`와 같습니다.

`pageTitle` (string, 필수)는 페이지 제목입니다.

`selector` (string, 필수)는 선택된 텍스트를 포함하는 요소의 DOM 경로입니다.

`range` (string, 필수)는 직렬화된 텍스트 범위로 형식은 `startOffset:endOffset,startOffset:endOffset{checksum}`입니다.

`createdFromCommentId` (string, 필수)는 이 채팅을 시작한 댓글의 ID입니다.

`broadcastId` (string, 필수)는 에코 효과를 방지하기 위한 실시간 동기화용 UUID입니다.

#### 응답

응답에는 API 상태와 새로 생성된 대화의 ID가 포함됩니다.

#### 예시 요청

[inline-code-attrs-start title = 'POST 요청 예시'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### 예시 응답

[inline-code-attrs-start title = 'POST 응답 예시'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### 대화 삭제

기존 텍스트 대화를 삭제합니다. 이 엔드포인트는 관리자 또는 중재자 권한이 필요하거나 대화가 인증된 사용자가 생성한 경우에만 허용됩니다.

#### 엔드포인트

[inline-code-attrs-start title = 'DELETE 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### 매개변수

`tenantId` (경로 매개변수, 필수)는 귀하의 FastComments Tenant ID입니다.

`chatId` (경로 매개변수, 필수)는 삭제하려는 대화의 ID입니다.

`broadcastId` (요청 본문, 필수)는 실시간 동기화를 위한 UUID입니다.

#### 예시 요청

[inline-code-attrs-start title = 'DELETE 요청 예시'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### 예시 응답

[inline-code-attrs-start title = 'DELETE 응답 예시'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### 요청 제한

이들 엔드포인트는 표준 FastComments API 요청 제한의 적용을 받습니다. 요청 제한 미들웨어는 정상적인 사용 패턴을 허용하면서 남용을 방지하기 위해 테넌트별로 적용됩니다.

### 오류 응답

모든 엔드포인트는 표준 HTTP 상태 코드를 반환합니다. 400 응답은 잘못된 요청 매개변수를 나타냅니다. 401 응답은 인증 실패를 의미합니다. 403 응답은 권한 부족을 나타냅니다. 404 응답은 대화를 찾을 수 없음을 의미합니다. 429 응답은 요청 한도 초과를 나타냅니다.

오류 응답에는 세부 정보가 포함된 JSON 본문이 포함됩니다:

[inline-code-attrs-start title = '오류 응답 예시'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### 사용량 추적

대화를 생성하면 `conversationCreateCount` 사용량 지표가 증가합니다. 모든 WebSocket 동기화 활동은 `pubSubMessageCount` 및 `pubSubBandwidth`를 증가시킵니다. 이러한 지표는 FastComments 대시보드의 사용량 분석에서 모니터링할 수 있습니다.

---