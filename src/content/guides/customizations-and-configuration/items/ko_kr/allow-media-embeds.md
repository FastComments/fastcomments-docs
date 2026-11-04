기본적으로 FastComments는 댓글에서 iframe을 허용하지 않습니다. 미디어 임베드를 활성화하면 댓글 작성자가 YouTube, Vimeo, SoundCloud, Spotify와 같은 신뢰할 수 있는 제공업체의 임베드 코드(`<iframe>` 스니펫)를 붙여넣을 수 있으며, 댓글 내에 인라인으로 렌더링됩니다.

보안을 위해 이것은 클라이언트 측 위젯 설정 플래그가 아닙니다. 각 댓글이 저장될 때 검증되는 서버 측 설정이므로, 페이지에서 켤 수 없습니다. 내장된 신뢰할 수 있는 제공업체 목록을 가리키는 iframe만 허용됩니다. 그 외의 iframe은 제거됩니다.

이 작업은 코드 없이 위젯 사용자 지정 페이지에서 수행할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='위젯 사용자 지정 페이지에서 미디어 임베드 설정이 켜져 있어 댓글 작성자가 신뢰할 수 있는 iframe 임베드를 붙여넣을 수 있습니다'; title='미디어 임베드 허용' app-screenshot-end]

### 자체 제공업체 추가하기

내장된 신뢰 목록에 없는 제공업체의 임베드를 허용하려면 같은 페이지의 "추가 임베드 도메인(Additional Embed Domains)" 필드에 해당 호스트명을 추가하세요. 이 호스트명은 내장 제공업체에 추가로 허용됩니다. 매칭은 정확히 일치해야 하므로 전체 호스트명을 포함해야 합니다(예: player.example.com). 목록에 없는 것은 계속 차단됩니다.

일반 댓글 입력창과 WYSIWYG 편집기 모두 임베드 붙여넣기를 지원합니다. WYSIWYG 편집기에서는 임베드가 제거 가능한 블록으로 삽입됩니다.