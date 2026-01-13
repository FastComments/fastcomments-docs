댓글 스레드를 검토하고 볼 때, 검토 중에 맥락을 파악하기 위해 스레드로 바로 이동할 수 있으면 좋습니다.

즉, 사용자의 흐름은 댓글 검토 페이지에서 시작하여 개별 댓글에서
해당 댓글이 있는 페이지로 이동한 다음, 그 페이지가 로드될 때까지 기다리고 댓글이 로드될 때까지 다시 기다린 다음 해당 댓글로 스크롤해야 합니다.

하지만 FastComments는 더 빠른 방법을 제공합니다. 댓글 검토 페이지에서는 각 댓글 옆 하단 오른쪽에 "댓글 보기" 버튼이 있습니다.

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

이 댓글에 답글이 있는 경우 버튼 텍스트는 대신 답글 수를 표시하지만, 클릭하면 같은 동작을 수행합니다.

이 버튼은 **댓글 스레드 뷰어**로 이동합니다.

댓글 스레드 뷰어는 FastComments에서 호스팅하는 작고 빠르게 로드되는 애플리케이션으로, 댓글이 있는 페이지의 댓글 스레드를 렌더링하고
해당 댓글로 스크롤합니다.

이를 통해 검토자는 다른 페이지가 로드될 때까지 기다리지 않고도 빠르게 필요한 맥락을 파악할 수 있습니다.

---