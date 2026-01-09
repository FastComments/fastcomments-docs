### API 개요

Image Chat는 이미지 대화를 프로그래밍 방식으로 관리할 수 있는 세 가지 REST API 엔드포인트를 제공합니다. 이 엔드포인트를 사용하면 브라우저 위젯을 사용하지 않고도 마커를 검색, 생성 및 삭제할 수 있습니다.

모든 API 엔드포인트는 인증이 필요하며 표준 FastComments API 패턴을 따릅니다. 이러한 엔드포인트는 API 키가 아닌 브라우저 쿠키를 통해 인증되는 공개 엔드포인트입니다.

### 기본 URL

모든 Image Chat API 엔드포인트는 다음 하위에 있습니다:

```
https://fastcomments.com/comment-image-chats
```

### 인증

이 엔드포인트들은 브라우저 쿠키를 통해 사용자를 인증합니다. API 키를 사용하지 않습니다. 사용자는 브라우저에서 FastComments에 로그인되어 있어야 이 엔드포인트에 접근할 수 있습니다.

### 모든 대화 가져오기

특정 이미지에 대한 모든 이미지 대화를 가져옵니다.

#### 엔드포인트

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### 매개변수

`tenantId` (경로 매개변수, 필수)는 귀하의 FastComments Tenant ID입니다.

`urlId` (쿼리 매개변수, 필수)는 대화를 검색하려는 이미지 식별자입니다.

#### 응답

응답에는 API 상태, 인증된 경우 현재 사용자 세션 정보, ID, URL 및 X/Y 좌표가 포함된 대화 목록 배열, 정리된 URL 식별자, 현재 사용자가 사이트 관리자 또는 중재자인지 여부를 나타내는 플래그, 라이브 동기화를 위한 WebSocket 연결 세부정보(`tenantIdWS`, `urlIdWS`, `userIdWS`)가 포함됩니다.

#### 예시 요청

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### 예시 응답

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### 대화 생성

이미지의 특정 위치에 대한 새 이미지 대화를 생성합니다.

#### 엔드포인트

```
POST /comment-image-chats/:tenantId
```

#### 매개변수

`tenantId` (경로 매개변수, 필수)는 귀하의 FastComments Tenant ID입니다.

요청 본문은 JSON이어야 하며 다음 필수 필드를 포함해야 합니다.

`urlId` (string, 필수)는 기본 페이지 식별자입니다.

`windowUrlId` (string, 필수)는 이미지 소스와 좌표가 결합된 URL로 예: `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, 필수)는 페이지 제목입니다.

`src` (string, 필수)는 이미지 소스 URL입니다.

`x` (number, 필수)는 이미지 너비 대비 X 좌표의 백분율(0-100)입니다.

`y` (number, 필수)는 이미지 높이 대비 Y 좌표의 백분율(0-100)입니다.

`createdFromCommentId` (string, 필수)는 이 채팅을 시작한 댓글의 ID입니다.

`broadcastId` (string, 필수)는 에코 효과를 방지하기 위한 라이브 동기화용 UUID입니다.

#### 응답

응답에는 API 상태와 새로 생성된 대화의 ID가 포함됩니다.

#### 예시 요청

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### 예시 응답

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### 대화 삭제

기존 이미지 대화를 삭제합니다. 이 엔드포인트는 관리자 또는 중재자 권한이 필요하거나, 인증된 사용자가 생성한 대화여야 합니다.

#### 엔드포인트

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### 매개변수

`tenantId` (경로 매개변수, 필수)는 귀하의 FastComments Tenant ID입니다.

`chatId` (경로 매개변수, 필수)는 삭제할 대화의 ID입니다.

`broadcastId` (요청 본문, 필수)는 라이브 동기화용 UUID입니다.

#### 예시 요청

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### 예시 응답

```json
{
  "status": "success"
}
```

### 좌표 시스템

X 및 Y 좌표는 이미지 치수에 대한 백분율로 저장됩니다. X는 왼쪽 가장자리에서의 수평 위치를 나타냅니다(0 = 왼쪽 가장자리, 100 = 오른쪽 가장자리). Y는 위쪽 가장자리에서의 수직 위치를 나타냅니다(0 = 위쪽 가장자리, 100 = 아래쪽 가장자리).

이러한 백분율 값은 정밀도를 위해 소수점을 포함할 수 있습니다. 예를 들어 x: 25.5는 이미지 왼쪽 가장자리로부터 25.5% 위치를 의미합니다.

### 속도 제한

이 엔드포인트들은 표준 FastComments API 속도 제한의 적용을 받습니다. 레이트 리밋 미들웨어는 정상적인 사용 패턴을 허용하면서 남용을 방지하기 위해 테넌트별로 적용됩니다.

### 오류 응답

모든 엔드포인트는 표준 HTTP 상태 코드를 반환합니다. 400 응답은 잘못된 요청 매개변수를 나타냅니다. 401 응답은 인증 실패를 의미합니다. 403 응답은 권한 부족을 나타냅니다. 404 응답은 대화를 찾을 수 없음을 의미합니다. 429 응답은 속도 제한 초과를 의미합니다.

오류 응답에는 세부 정보를 포함한 JSON 본문이 포함됩니다:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### 사용량 추적

대화를 생성하면 `conversationCreateCount` 사용량 메트릭이 증가합니다. 모든 WebSocket 동기화 활동은 `pubSubMessageCount` 및 `pubSubBandwidth`를 증가시킵니다. 이러한 메트릭은 FastComments 대시보드의 사용 분석에서 모니터링할 수 있습니다.