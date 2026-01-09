[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

기본적으로 라이브 댓글 기능이 활성화되어 있습니다. 즉, 댓글이 추가, 삭제, 수정되거나 고정되면 해당 변경 사항이
댓글 스레드를 보고 있는 모든 사용자에게 동시에 표시됩니다.

그러나 기본적으로 새 댓글은 "새 댓글 2개 보기"와 유사한 텍스트를 가진 동적으로 표시되는 버튼 아래에 나타납니다.

If the new comments are replies directly to the page, the button will show at the top of the comment thread. If they are replies to a particular comment, 
the button will show under that comment.

이는 사용자가 스크롤바를 잡으려 할 때 페이지 크기가 계속 변경되어 좌절감을 줄 수 있는 상황을 방지하기 위한 것입니다.

For some use cases, like live bidding, or online events, this is not the desired behavior - you may want the commenting widget to be
more like a "채팅" 상자 where new comments "즉시 표시".

Hence, the name of the flag that enables that feature: **showLiveRightAway**.

We can turn it on as follows:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---