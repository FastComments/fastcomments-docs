---
기본적으로 FastComments 페이지 크기는 `30`입니다. 여기에는 스레드의 답글도 포함됩니다.

페이지 크기는 [Widget Configuration UI](https://fastcomments.com/auth/my-account/customize-widget)에서 `10`부터 `200`까지 다양한 크기로 사용자 지정할 수 있습니다.

페이지 크기를 변경하면 계정의 모든 댓글 스레드를 다시 계산해야 합니다. 이 작업은 몇 분 정도 걸릴 수 있습니다.

페이지는 서버 측에서 계산되므로 클라이언트 측 위젯에서는 이 설정을 구성할 수 없습니다.

아래에 예시 구성이 표시됩니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.page-size'; alt='위젯 사용자 지정 페이지에서 10부터 200까지의 값을 선택할 수 있는 페이지 크기 선택기'; title='맞춤 페이지 크기' app-screenshot-end]

페이지 크기는 전역적으로, 도메인별로 또는 페이지별로 다양한 사용자 지정 규칙을 만들어 맞춤 설정할 수 있습니다.

이는 플랫폼을 통해 댓글을 표시하는 모든 클라이언트, 통합 및 프레임워크에 영향을 미칩니다.

---