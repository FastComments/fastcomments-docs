[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

사용자가 FastComments에 처음 댓글을 달면 우리는 그들의 아바타를 <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>에서 가져오려고 시도합니다.

하지만 아바타를 찾지 못하거나 사용자가 계정에 아바타를 설정하지 않은 경우, 우리는 정적인 기본 아바타 이미지를 표시합니다.

자신만의 정적 아바타 이미지를 지정하려면 *defaultAvatarSrc* 설정을 사용할 수 있습니다.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = '기본 아바타 재정의'; code-example-end]

코드를 사용하지 않고도 이 작업을 수행할 수 있습니다. 위젯 사용자 정의 페이지에서 "Default Avatar" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='위젯 사용자 정의 페이지의 기본 아바타 섹션으로, 대체 아바타 이미지 URL을 설정합니다'; title='기본 아바타 사용자 정의' app-screenshot-end]

SSO와 같이 특정 사용자의 아바타를 정의하는 방법은 별도의 섹션에서 다룹니다.