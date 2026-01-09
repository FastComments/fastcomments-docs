---
기본적으로 FastComments는 다음과 같이 링크를 렌더링합니다: [https://exmaple.com](https://exmaple.com) - 여기서 링크 URL은 클릭 가능한
HTML 앵커 태그가 됩니다.

일부 사이트는 예를 들어 사기꾼을 억제하기 위해 이를 비활성화하고 싶을 수 있습니다. 우리는 이를 `Comment HTML Rendering Option`을 `Links as Text`로 설정하여 제공합니다.

이 설정은 위젯 사용자화 페이지에서 코드 없이 도메인 전체 또는 개별 페이지에 대해 사용자화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option']; selector = '.comment-html-rendering-mode'; title='Render Links as Text' app-screenshot-end]

---