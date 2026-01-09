[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments의 서식 기능은 텍스트 주변에 `<b></b>` 같은 눈에 보이는 앵커 태그를 추가하는 방식으로 이루어집니다. 툴바를 클릭하거나 단축키를 사용하면 이러한 처리가 자동으로 적용됩니다. 그러나 일부 커뮤니티는 앵커 태그 없이 서식을 적용하도록 선택할 수 있습니다. 이를 WYSIWYG(보이는 것이 실제 결과인) 편집기를 활성화한다고 합니다. 이 편집기는 기본 편집기와 외형상 동일하지만, 눈에 보이는 앵커 태그 없이 사용자가 텍스트를 굵게, 밑줄 등으로 꾸밀 수 있도록 추가 코드를 로드합니다.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

이 작업은 코드 없이도 할 수 있습니다. 위젯 맞춤 설정 페이지에서 '고급 서식 사용' 옵션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]