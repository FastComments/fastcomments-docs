기본적으로 FastComments는 모든 외부 사이트로의 링크를 허용합니다.

This can be restricted to instead a desired list of sites, or domains. Attempting to post a link to a site, or domain,
정의된 목록에 없는 사이트나 도메인으로 링크를 게시하려고 하면 사용자에게 오류가 표시됩니다.

이 검증은 댓글 위젯과 API에만 적용됩니다. 임포트에는 영향을 주지 않습니다.

이 설정은 코드 없이 위젯 사용자화 페이지에서 수행됩니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; title='Restrict External Link Domains' app-screenshot-end]