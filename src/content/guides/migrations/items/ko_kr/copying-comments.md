데이터를 이동해야 하는 경우, FastComments는 페이지와 아티클 간에 댓글을 이동할 수 있는 셀프 서비스 도구를 제공합니다.

댓글 복사 페이지 양식은 다음과 같이 생겼습니다:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='From URL ID 필드와 To URL ID 및 URL 필드가 있는 댓글 복사 양식'; title='댓글 복사 양식' app-screenshot-end]

### "From" 필드 작성하기

댓글을 어디에서 이동할지 결정하려면, 소스 `URL ID`만 알면 됩니다.

댓글 위젯 구성에서 `urlId`에 값을 전달하지 않는 경우, 이는 페이지 URL의 "클린" 버전이 됩니다.

댓글 내보내기를 통해 댓글이 가지고 있는 `URL ID` 값을 확인할 수 있습니다.

### "To" 필드 작성하기

댓글을 어디로 이동할지 결정하려면, 대상 `URL ID`와 `URL`을 알아야 합니다.

`URL ID`는 댓글이 속할 버킷입니다. `URL` 필드는 이메일 및 검토 도구에서 댓글로 직접 이동할 수 있도록 사용됩니다.

#### WordPress

WordPress를 사용하는 경우, 마이그레이션 도구의 To/From `URL ID` 필드에 URL 대신 아티클 ID를 입력하면 됩니다.