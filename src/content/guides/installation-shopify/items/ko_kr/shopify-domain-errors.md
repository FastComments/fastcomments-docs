Shopify App Store에서 FastComments를 설치한 경우, 상점 도메인이 테넌트의 승인된 도메인에 자동으로 추가되므로 도메인 오류가 표시되지 않아야 합니다. 이 페이지는 수동 설치 경로를 통해 설치했거나, 앱 설치 시점에 Shopify에 등록되지 않은 커스텀 도메인에서 스토어프론트가 제공되는 경우에 적용됩니다.

다음과 같은 권한 오류가 발생할 수 있습니다:

<div class="screenshot white-bg">
    <div class="title">도메인 구성 누락</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-webflow-step-5.png" alt="도메인 구성 누락" />
</div>

이는 FastComments가 위젯이 로드되는 도메인을 귀하의 테넌트에 대해 승인된 도메인으로 인식하지 못함을 의미합니다.

해결하려면 FastComments 계정에 도메인을 추가하세요: [도메인 구성](https://fastcomments.com/auth/my-account/configure-domains).