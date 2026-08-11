---
[related-parameter-start name = 'disableProfiles'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자가 아바타를 클릭하면 해당 사용자의 프로필을 표시합니다.

하지만 이 기능을 비활성화할 수 있습니다:

[code-example-start config = {disableProfiles: true}; linesToHighlight = [6]; title = 'Disable Profiles'; code-example-end]

코드를 사용하지 않고도 이 작업을 수행할 수 있습니다. 위젯 커스터마이징 페이지에서 "Disable Profiles" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profiles']; selector = '.disable-profiles'; alt='Disable Profiles 체크박스가 선택된 위젯 커스터마이징 페이지로, 아바타를 클릭해도 더 이상 프로필이 열리지 않습니다'; title='프로필 비활성화' app-screenshot-end]

---