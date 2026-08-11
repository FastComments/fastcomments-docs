[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

우리는 **enableSpoilers** 플래그를 true로 설정하여 스포일러 지원을 활성화할 수 있습니다:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = '스포일러 활성화'; code-example-end]

코드를 사용하지 않고도 이 작업을 수행할 수 있습니다. 위젯 커스터마이징 페이지에서 "Enable Spoilers" 옵션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='위젯 커스터마이징 페이지에서 Enable Spoilers 체크박스를 선택하여 편집기에 SPOILER 버튼을 추가한 모습'; title='스포일러 활성화' app-screenshot-end]

텍스트를 강조 표시하고 이제 보이는 `SPOILER` 버튼을 클릭하면, 텍스트가 마스크 처리되어 사용자가 마우스를 올릴 때까지 숨겨집니다. 다크 모드에서는 동일한 방식을 사용하지만, 다크 모드에 더 잘 맞는 다른 색상을 사용합니다.

이 기능은 WYSIWYG 편집기와도 호환됩니다.