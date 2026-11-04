WordPress에서 사이트를 이전하고 새 사이트에 FastComments를 사용하려는 경우 WordPress 플러그인은 필요하지 않습니다. WordPress에서 댓글을 내보낸 후, FastComments 대시보드의 [가져오기 페이지](https://fastcomments.com/auth/my-account/manage-data/import)에서 파일을 업로드하세요.

두 가지 WordPress 내보내기 형식을 지원합니다.

### WordPress XML (권장)

WordPress의 내장 내보내기 기능에서 생성되는 파일이므로 추가 플러그인이 필요하지 않습니다.

1. WordPress 관리자 화면에서 `Tools -> Export`로 이동합니다.
2. `All content`를 선택하고 `Download Export File`을 클릭합니다.
3. FastComments [가져오기 페이지](https://fastcomments.com/auth/my-account/manage-data/import)에서 `WordPress (.xml)`을 선택하고 파일을 업로드합니다.

각 댓글은 댓글이 작성된 게시물의 URL에 연결되며, 이 URL은 이미 파일에 포함되어 있습니다.

가져오기 시 작성자 이름, 이메일, 웹사이트, 날짜, 내용, 답글 스레드 구조, 댓글 승인 여부가 유지됩니다. 댓글 작성자 아바타는 Gravatar에서 가져옵니다. 투표는 이 형식에 포함되지 않습니다.

### WordPress CSV

[WebToffee의 WordPress Comments Import & Export 플러그인](https://wordpress.org/plugins/comments-import-export-woocommerce/)에서 생성되는 파일입니다.

1. WordPress 관리자 화면에 플러그인을 설치하고 댓글을 CSV로 내보냅니다.
2. 각 `comment_post_ID` 값을 게시물의 URL로 교체합니다.
3. FastComments [가져오기 페이지](https://fastcomments.com/auth/my-account/manage-data/import)에서 `WordPress (.csv)`를 선택하고 파일을 업로드합니다.

각 댓글은 `comment_post_ID` 열에 연결됩니다. WordPress는 이 열에 게시물 ID를 채우지만 새 사이트에는 WordPress 게시물 ID가 없으므로, 2단계에서 이를 URL로 교체합니다.

가져오기 시 작성자 이름, 이메일, 웹사이트, 날짜, 내용, 답글 스레드 구조, 댓글 승인 여부가 유지됩니다. 댓글 작성자 아바타는 Gravatar에서 가져옵니다. 또한 파일에 포함되어 있는 경우 WordPress의 스팸 플래그와 wpDiscuz의 좋아요/싫어요도 유지됩니다.

### 새 페이지에 댓글 매칭하기

새 사이트가 WordPress 사이트와 동일한 URL을 유지하는 경우, 추가 설정 없이도 댓글이 해당 페이지에 표시됩니다.

도메인이 변경되는 경우, 가져오기 후 [도메인 마이그레이션 도구](/guide-migrations.html#migrating-domains)를 실행하세요. 개별 페이지 URL이 변경되는 경우, 이전 URL에서 새 URL로 [각 페이지를 마이그레이션](/guide-migrations.html#migrating-pages)할 수 있습니다.

댓글 위젯의 [urlId](/guide-customizations-and-configuration.html#url-id) 필드에 전달하는 값에서 도메인을 제거하는 것과 같은 대량 페이지 마이그레이션의 경우, [지원 티켓을 열어](https://fastcomments.com/auth/my-account/help) 주시면 저희가 처리해 드립니다.

### 전환하기 전에

가져오기는 원하는 만큼 여러 번 실행할 수 있습니다. 동일한 파일을 다시 가져와도 [중복이 생성되지 않으므로](/guide-migrations.html#importing-data), 새 사이트를 테스트하기 위해 한 번 가져온 후, 전환 직전에 최신 댓글로 다시 가져올 수 있습니다.

1GB보다 큰 내보내기 파일의 경우 [지원팀에 문의](https://fastcomments.com/auth/my-account/help)하세요.

새 사이트에 FastComments를 추가하는 방법은 [설치 가이드](/guide-installation.html)를 참조하세요.