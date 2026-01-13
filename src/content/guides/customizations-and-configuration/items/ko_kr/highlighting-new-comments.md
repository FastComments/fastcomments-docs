FastComments는 새 댓글을 강조 표시하는 여러 방법을 제공합니다.

First and foremost, by default comments that triggered an in-app notification (replies, replies in same thread, or comments on a page
that you're subscribed to), will automatically be highlighted with the user's avatar glowing slightly. 색상은 CSS를 사용하여
`is-unread` 클래스로 사용자화할 수 있습니다.

최근 24시간 내에 작성된 댓글에는 스타일링에 사용할 수 있는 `24hr` 클래스가 적용됩니다.

마지막으로, 사용자 세션에 새로 나타나는 실시간 댓글은 애니메이션을 통해 몇 초 동안 강조 표시됩니다. 이는
`is-live` CSS 클래스로 이루어지며 역시 사용자화할 수 있습니다.