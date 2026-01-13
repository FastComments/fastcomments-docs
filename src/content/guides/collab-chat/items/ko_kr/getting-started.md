### 빠른 시작

Collab Chat을 시작하는 것은 간단합니다. FastComments Collab Chat 스크립트, 주석을 달고자 하는 텍스트를 포함한 HTML 요소, 그리고 Tenant ID가 포함된 구성 객체가 필요합니다.

### 설치

페이지에 Collab Chat 스크립트를 추가하세요:

[inline-code-attrs-start title = 'Collab Chat 스크립트 로드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### 기본 구현

간단한 예제는 다음과 같습니다:

[inline-code-attrs-start title = '기본 Collab Chat 구현'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- 콘텐츠 컨테이너 -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Collab Chat 스크립트 로드 -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Collab Chat 초기화 -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

실제 FastComments Tenant ID로 `'demo'`를 바꾸세요(이미 설정되어 있지 않은 경우). Tenant ID는 [FastComments 대시보드](https://fastcomments.com/auth/my-account/api-secret)에서 찾을 수 있습니다.

### 작동 방식

초기화되면 사용자는 대상 요소 내의 임의의 텍스트를 선택할 수 있습니다. 짧은 지연 후(데스크탑에서는 3.5초), 토론을 시작할 수 있는 프롬프트가 나타납니다. 토론이 생성되면 텍스트에 시각적 하이라이트가 표시됩니다. 다른 사용자는 하이라이트에 마우스를 올리거나 클릭하여 토론을 보고 참여할 수 있습니다. 모든 토론은 모든 방문자에게 실시간으로 동기화됩니다.

### 라이브 데모

Collab Chat의 동작을 [라이브 데모 페이지](https://fastcomments.com/product/collab-chat)에서 확인할 수 있습니다.

### 다음 단계

기본 기능이 작동하면 구성 옵션 가이드에서 외관과 동작을 사용자화할 수 있습니다. 텍스트 선택 동작 가이드를 확인하여 텍스트 선택 방식에 대해 이해하세요. 커스터마이제이션 가이드에서 스타일링 및 다크 모드 지원에 대해 알아보세요. 고급 통합의 경우 API 레퍼런스를 살펴보세요.

### 프론트엔드 라이브러리

모든 FastComments 프론트엔드 라이브러리(react, vue, angular 등)는 Collab Chat을 포함하고 있습니다.