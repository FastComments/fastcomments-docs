데이터를 이동해야 하는 경우, FastComments는 페이지와 기사 간에 댓글을 이동할 수 있는 셀프 서비스 도구를 제공합니다.

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Filling out the "From" Fields

댓글을 어디에서 이동할지 결정하려면, 소스 `URL ID`만 알면 됩니다.

댓글 위젯 구성에서 `urlId` 값을 전달하지 않는 경우, 이것은 페이지 URL의 "정규화된" 버전이 됩니다.

댓글들이 가진 `URL ID` 값을 내보내기하여 확인할 수 있습니다.

### Filling out the "To" Fields

댓글을 어디로 이동할지 결정하려면 대상 `URL ID`와 `URL`을 알아야 합니다.

`URL ID`는 댓글이 들어갈 버킷이 됩니다. `URL` 필드는 이메일 및 모더레이션 도구에서 댓글로 직접 이동할 수 있도록 사용됩니다.

#### WordPress

WordPress를 사용하고 있다면, 예를 들어 마이그레이션 도구의 To/From `URL ID` 필드에 URL 대신 기사 ID를 입력하면 됩니다.