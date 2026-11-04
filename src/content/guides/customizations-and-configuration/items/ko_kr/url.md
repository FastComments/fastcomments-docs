[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

알림 이메일을 보내거나 관리 페이지와 같은 사용자 인터페이스에서 댓글을 렌더링할 때, 댓글에서 해당 댓글이 달린 페이지로 연결할 수 있으면 유용합니다.

URL ID가 항상 ID가 아니라면, URL을 다른 곳에 저장해야 합니다. 그것이 바로 "url" 속성의 용도이며, 다음과 같이 정의됩니다.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = '사용자 정의 URL 정의'; code-example-end]

일반적인 사용 사례는 댓글 스레드를 게시글과 같은 식별자에 연결한 다음, 특정 페이지로 다시 연결하는 것입니다. 예를 들면:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = '사용자 정의 URL과 URL ID 함께 정의'; code-example-end]

URL은 일반적인 마케팅 매개변수가 제거되지 않습니다. 기본적으로 현재 페이지 URL이 그대로 댓글과 함께 저장되는 URL이 됩니다.