[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

스포일러 지원은 **enableSpoilers** 플래그를 true로 설정하여 활성화할 수 있습니다:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

이 작업은 코드 없이도 할 수 있습니다. 위젯 사용자 지정 페이지에서 "Enable Spoilers" 옵션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

텍스트를 강조 표시한 후 표시된 `SPOILER` 버튼을 클릭하면, 사용자가 마우스를 올릴 때까지 텍스트가 가려집니다. 다크 모드에서는 동일한 동작을 수행하지만 다크 모드에 더 어울리는 다른
색상을 사용합니다.

이는 WYSIWYG 편집기와도 호환됩니다.