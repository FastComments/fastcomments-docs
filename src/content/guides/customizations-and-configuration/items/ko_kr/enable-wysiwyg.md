---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments의 서식 기능은 텍스트 주변에 `<b></b>`와 같은 보이는 앵커 태그를 추가하여 수행됩니다. 툴바를 클릭
하거나 단축키를 사용하면 이 작업이 자동으로 이루어집니다. 그러나 일부 커뮤니티에서는 앵커 태그 없이 서식을 사용하도록 선택하고 싶을 수 있습니다. 이를
WYSIWYG(what you see is what you get) 편집기라고 합니다. 이 편집기는 기본 편집기와 정확히 동일하게 보이지만, 추가 코드를 로드하여
사용자가 보이는 앵커 태그 없이도 텍스트를 굵게, 밑줄 등으로 서식 지정할 수 있게 합니다.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'WYSIWYG 편집 활성화'; code-example-end]

코드 없이도 이 작업을 수행할 수 있습니다. 위젯 커스터마이징 페이지에서 "Enable Advanced Formatting" 옵션을 확인하십시오.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='WYSIWYG 편집기를 켜기 위해 Enable Advanced Formatting 체크박스가 선택된 위젯 커스터마이징 페이지'; title='WYSIWYG 활성화' app-screenshot-end]

---