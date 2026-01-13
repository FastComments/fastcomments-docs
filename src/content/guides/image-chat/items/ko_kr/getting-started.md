### 사용 사례

Image Chat는 팀이 목업이나 스크린샷의 특정 요소에 대해 논의해야 할 때 디자인 피드백에 매우 적합합니다. 제품 리뷰 사이트에서는 고객이 제품 사진에서 보이는 특정 기능에 대해 토론할 수 있습니다. 교육 플랫폼은 다이어그램, 지도 또는 과학 이미지에 대해 토론하는 데 사용할 수 있습니다. 사진 갤러리는 위치별 주석으로 인터랙티브해질 수 있습니다. 부동산 사이트에서는 방문자가 부동산 사진에서 보이는 특정 특징에 대해 논의할 수 있습니다.

### 빠른 시작

Image Chat 시작은 간단합니다. FastComments Image Chat 스크립트, 이미지가 포함된 이미지 요소나 컨테이너, 그리고 Tenant ID가 포함된 구성 객체가 필요합니다.

### 설치

페이지에 Image Chat 스크립트를 추가하세요:

[inline-code-attrs-start title = 'Image Chat 스크립트 로드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### 기본 구현

Here's a minimal example:

[inline-code-attrs-start title = '기본 Image Chat 구현'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- 이미지 -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Image Chat 스크립트 로드 -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Image Chat 초기화 -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments 대시보드](https://fastcomments.com/auth/my-account).

### 작동 방식

초기화되면 사용자는 이미지의 아무 곳이나 클릭할 수 있습니다. 클릭이 발생하면 해당 위치에 시각적 정사각형 마커가 나타나고 채팅 창이 열립니다. 다른 사용자들도 모든 마커를 보고 클릭하여 해당 토론을 보거나 참여할 수 있습니다. 모든 토론은 방문자 전체에 걸쳐 실시간으로 동기화됩니다.

위젯은 백분율 기반 위치 지정을 사용하므로 이미지 크기가 조정되거나 다른 화면 크기에서 보더라도 마커는 올바른 위치에 유지됩니다.

### 라이브 데모

Image Chat 작동 모습을 [라이브 데모 페이지](https://fastcomments.com/product/image-chat)에서 확인할 수 있습니다.

### 다음 단계

기본 기능이 작동하면 구성 옵션 가이드에서 모양과 동작을 사용자 지정할 수 있습니다. 반응형 디자인 가이드를 확인하여 백분율 기반 위치 지정이 어떻게 작동하는지 이해하세요. 맞춤 설정 가이드에서 스타일링 및 다크 모드 지원에 대해 알아보세요. 고급 통합은 API 레퍼런스를 참조하세요.

### 프런트엔드 라이브러리

모든 FastComments 프런트엔드 라이브러리(react, vue, angular 등)에 Image Chat이 포함되어 있습니다.