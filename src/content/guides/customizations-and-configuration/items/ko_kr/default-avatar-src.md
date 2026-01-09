[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

사용자가 FastComments로 처음 댓글을 작성할 때, 우리는 그들의 아바타를 <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>에서 가져오려고 시도합니다.

그러나 아바타를 찾지 못하거나 사용자가 계정에서 아바타를 설정하지 않은 경우, 정적 기본 아바타 이미지를 렌더링합니다.

자체 정적 아바타 이미지를 지정하려면 *defaultAvatarSrc* 설정을 사용할 수 있습니다.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

이것은 코드 없이도 할 수 있습니다. 위젯 커스터마이제이션 페이지에서 '기본 아바타' 섹션을 참조하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

특정 사용자(예: SSO)를 위한 아바타 정의는 별도의 섹션에서 다룹니다.

---