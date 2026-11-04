### 사용 사례

Image Chat는 팀이 목업이나 스크린샷의 특정 요소에 대해 논의해야 하는 디자인 피드백에 매우 유용합니다. 제품 리뷰 사이트에서는 고객이 제품 사진에 보이는 특정 기능에 대해 논의할 수 있습니다. 교육 플랫폼에서는 다이어그램, 지도 또는 과학 이미지를 논의하는 데 활용할 수 있습니다. 사진 갤러리는 위치별 댓글 기능으로 인터랙티브하게 만들 수 있습니다. 부동산 사이트에서는 방문자가 매물 사진에 보이는 특정 특징에 대해 논의할 수 있습니다.

### 빠른 시작

Image Chat를 시작하는 것은 간단합니다. FastComments Image Chat 스크립트, 이미지가 있는 이미지 요소 또는 컨테이너, 그리고 Tenant ID가 포함된 설정 객체가 필요합니다.

### 설치

페이지에 Image Chat 스크립트를 추가하세요:

[inline-code-attrs-start title = 'Image Chat 스크립트 로드하기'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### 기본 구현

다음은 최소한의 예제입니다:

[inline-code-attrs-start title = '기본 Image Chat 구현'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Your image -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Load the Image Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Initialize Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

아직 변경하지 않았다면 `'demo'`를 실제 FastComments Tenant ID로 교체하세요. Tenant ID는 [FastComments 대시보드](https://fastcomments.com/auth/my-account)에서 확인할 수 있습니다.

### 작동 방식

초기화가 완료되면 사용자는 이미지의 어느 곳이든 클릭할 수 있습니다. 클릭이 발생하면 해당 위치에 시각적인 사각형 마커가 나타나고 채팅 창이 열립니다. 다른 사용자는 모든 마커를 볼 수 있으며, 마커를 클릭하여 해당 토론을 보거나 참여할 수 있습니다. 모든 토론은 모든 방문자 간에 실시간으로 동기화됩니다.

이 위젯은 백분율 기반 위치 지정을 사용하므로, 이미지 크기가 조정되거나 다양한 화면 크기에서 보더라도 마커는 올바른 위치에 유지됩니다.

### 라이브 데모

[라이브 데모 페이지](https://fastcomments.com/product/image-chat)에서 Image Chat가 작동하는 모습을 확인할 수 있습니다.

### 다음 단계

기본적인 사용법을 익혔다면, Configuration Options 가이드에서 모양과 동작을 사용자 지정할 수 있습니다. Responsive Design 가이드를 확인하여 백분율 기반 위치 지정의 작동 방식을 이해하세요. Customization 가이드에서 스타일링과 다크 모드 지원에 대해 알아보세요. 고급 통합을 위해서는 API Reference를 살펴보세요.

### 프론트엔드 라이브러리

모든 FastComments 프론트엔드 라이브러리(react, vue, angular 등)에는 Image Chat가 포함되어 있습니다.