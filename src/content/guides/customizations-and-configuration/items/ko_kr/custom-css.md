[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments는 사용자 정의가 가능하도록 설계되었습니다. 보안상의 이유로 댓글 위젯 자체가 iframe 안에서 실행되기 때문에, 맞춤 스타일을 적용하려면 두 가지 방법 중 하나를 따라야 합니다.

첫 번째이자 가장 쉬운 방법은 저희가 권장하는 **위젯 커스터마이징 페이지**([widget customization page](https://fastcomments.com/auth/my-account/customize-widget))를 이용하는 것입니다.

위젯 커스터마이징 페이지에서 “고급 옵션 표시” 섹션을 확인하면 “Custom CSS” 라벨이 붙은 영역이 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='위젯 커스터마이징 페이지의 고급 옵션 표시 아래에 있는 Custom CSS 편집기'; title='Custom CSS 입력 영역' app-screenshot-end]

이 방법의 장점:
1. 입력된 CSS는 사용자에게 전달되기 전에 최소화(minify)되며, 편집 UI에서 포맷이 일관되게 유지됩니다.
2. 위젯 커스터마이징 UI의 모든 이점을 활용할 수 있어, 예를 들어 사이트마다 댓글 위젯을 쉽게 다르게 커스터마이징할 수 있습니다.
3. 저희가 댓글 위젯을 업데이트할 때, 여러분의 맞춤 스타일도 릴리스 프로세스의 일부로 테스트됩니다.

두 번째 방법은 위젯 설정에 **customCSS** 매개변수를 지정하는 것입니다:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

하지만 이 방법에는 *제한 사항*이 있습니다:
1. 헤더 크기 제한으로 인해 서버가 요청을 거부하기 전에 전달할 수 있는 맞춤 CSS 양에 제한이 있습니다.
2. 인프라와 빌드 시스템에서 맞춤 CSS를 직접 관리해야 합니다. 이는 단점이라기보다 장점이 될 수도 있습니다.
3. 이 경우 맞춤 CSS가 네트워크를 통해 **두 번** 전송되는 추가 오버헤드가 발생합니다(서버로 전송된 뒤 iframe 콘텐츠에 다시 전달). 대부분의 페이로드 크기에서는 눈에 띄지 않습니다.
4. 일반적인 최적화 방법은 CSS를 최소화하여 네트워크 전송 크기를 줄이는 것이지만, 이 방법을 사용할 경우 직접 처리해야 합니다.
5. 저희가 위젯을 업데이트할 때 맞춤 CSS가 테스트되지 않습니다.

### 외부 CSS 파일

`@import`를 사용하여 위젯이 외부 파일을 가져오도록 할 수 있습니다!

`@import`는 커스터마이징 규칙 안에 넣는 것이 권장됩니다. 이렇게 하면 댓글 위젯에 변경이 필요할 때 자동화 도구를 사용해 설정을 검증할 수 있습니다. 예를 들어, 위젯 커스터마이징 UI에서 **Custom CSS**에 `Advanced`를 클릭하고 다음과 같이 입력합니다:

    @import url(https://example.com/styles.css);

#### 코드 내 - 권장되지 않음

`customCSS` 속성을 통해 외부 CSS 파일을 로드할 수도 있습니다:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

하지만 이렇게 하면 저희가 해당 CSS를 테스트할 수 없다는 점을 기억하세요.

### 사용자 프로필 모달 스타일링

사용자 프로필 모달도 맞춤 CSS로 스타일링할 수 있습니다. 다만, 사용자 프로필에 맞춤 스타일이 적용되도록 하려면 모든 CSS 선택자 앞에 `.user-profile` 접두사를 붙여야 합니다. 이 접두사가 없으면 사용자 프로필 모달에 대한 맞춤 스타일이 무시됩니다.

예시:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### 이전 버전 호환성

FastComments에서는 고객이 댓글 위젯을 커스터마이징한다는 점을 잘 알고 있습니다. 이는 설계상의 의도이며, 저희 제품이 고객 제품의 디자인 일관성을 해치는 상황을 원치 않습니다.

이 중요한 부분을 위해 저희는 각 릴리스마다 고객별로 댓글 위젯 변경 사항을 검토할 수 있는 빌드 파이프라인을 운영하고 있습니다.

작은 문제가 발견되면 계정을 업데이트하여 릴리스가 원활히 진행되도록 하고, 큰 파괴적 변경이 감지되면 릴리스를 중단할 수 있습니다.