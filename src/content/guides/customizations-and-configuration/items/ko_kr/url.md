[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

알림 이메일을 보내거나, 모더레이션 페이지와 같은 사용자 인터페이스에 댓글을 렌더링할 때, 댓글에서 해당 댓글이 달린 페이지로 링크를 걸 수 있으면 유용합니다.

If URL ID isn't always an ID, then we have to store the URL some place else. That's what the "url" property is for, defined as follows.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

일반적인 사용 사례는 댓글 스레드를 기사와 같은 식별자에 연결한 다음, 특정 페이지로 다시 링크하는 것입니다. 예를 들어:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL은 일반적인 마케팅 파라미터가 제거되지 않습니다. 기본적으로 현재 페이지의 URL이 무엇이든, 그 URL이 댓글과 함께 저장됩니다.