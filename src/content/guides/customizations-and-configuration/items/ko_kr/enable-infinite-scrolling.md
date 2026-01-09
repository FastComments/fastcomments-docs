[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments 위젯은 모든 보이는 댓글이 보이도록 세로 크기를 자동으로 조정합니다. 페이지 매김은 현재 페이지 끝에 있는 "다음 보기" 버튼으로 이루어지며, 이는 대부분의 사용자에게 가장 쾌적한 상호작용이라고 판단했습니다.

그러나 무한 스크롤이 더 선호되는 경우도 있습니다. 예를 들어, 당사 Stream Chat 제품에서 이 기능을 사용합니다.

"다음 보기" 버튼을 숨기고 **enableInfiniteScrolling** 플래그를 true로 설정하여 무한 스크롤로 전환할 수 있습니다:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

또한 이를 위해 사용자 지정 CSS를 추가해야 합니다. 예를 들어 스크롤을 활성화하려면 `.comments` 선택자에 대한 사용자 지정 CSS를 추가하세요:

[inline-code-attrs-start title = '무한 스크롤 활성화'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

전체 작동 예시는 다음과 같습니다:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

위 예제에서는 `customCSS` 속성을 사용했지만, 성능상의 이유로 위젯 구성 UI를 대신 사용하는 것이 권장됩니다. [사용자 지정 CSS 문서 참조.](/guide-customizations-and-configuration.html#custom-css)