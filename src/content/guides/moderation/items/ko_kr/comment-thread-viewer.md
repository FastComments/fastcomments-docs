---
댓글 스레드를 검토하고 볼 때, 검토 중에 컨텍스트를 얻기 위해 스레드로 바로 이동할 수 있으면 좋습니다.

이는 사용자의 흐름이 댓글 검토 페이지에서 시작하여 개별 댓글에서 해당 댓글이 포함된 페이지로 이동하고, 페이지가 로드될 때까지 기다린 뒤, 댓글이 로드될 때까지 기다린 후, 그 댓글까지 스크롤해야 함을 의미합니다.

하지만 FastComments는 더 빠른 방법을 제공합니다. 댓글 검토 페이지에서 각 댓글 옆에 오른쪽 하단에 "View Comment" 버튼이 있습니다.

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; alt='검토 목록에 있는 단일 댓글이며, 오른쪽 하단에 View Comment 버튼이 있습니다.'; title='댓글' app-screenshot-end]

이 댓글에 답글이 있는 경우, 버튼 텍스트는 대신 답글 수를 표시하지만 클릭하면 동일한 동작을 수행합니다.

이 버튼을 클릭하면 **Comment Thread Viewer** 로 이동합니다.

Comment Thread Viewer는 FastComments가 호스팅하는 작고 빠르게 로드되는 애플리케이션으로, 해당 댓글이 있는 페이지의 댓글 스레드를 렌더링하고 그 댓글까지 스크롤합니다.

이를 통해 검토자는 다른 페이지가 로드될 때까지 기다리지 않고도 필요한 컨텍스트를 빠르게 수집할 수 있습니다.
---