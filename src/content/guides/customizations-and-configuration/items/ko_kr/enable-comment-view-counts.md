[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 각 댓글을 누가 보았는지 추적하거나 이에 대한 통계를 제공하지 않습니다.

하지만 이 기능을 활성화하면 사용자가 댓글로 스크롤할 때 시스템이 이를 추적하기 시작합니다.

이렇게 되면 각 댓글에 표시된 눈 아이콘 옆의 카운트가 증가합니다. 카운트는 실시간으로 업데이트되며 사용자의 로케일에 따라 축약되어 표시됩니다.

다음과 같이 **enableViewCounts** 플래그를 true로 설정하여 이 기능을 활성화할 수 있습니다:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

이 설정은 위젯 맞춤화 페이지에서 코드 없이도 사용자화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

We track the user id* that viewed the comment, so that if you view the comment again it does not increment. If you view the comment again
after two years, the count will increment more.

- *Note: or the anon session id, or the user's IP as a hashed value.