By default FastComments does not allow iframes in comments. When you enable media embeds, commenters can paste the embed code (the `<iframe>` snippet) from trusted providers like YouTube, Vimeo, SoundCloud, and Spotify, and it will render inline in the comment.

기본적으로 FastComments는 댓글에서 iframe을 허용하지 않습니다. 미디어 임베드를 활성화하면, 댓글 작성자는 YouTube, Vimeo, SoundCloud, Spotify와 같은 신뢰할 수 있는 제공업체의 임베드 코드( `<iframe>` 스니펫)를 붙여넣을 수 있으며, 댓글에 인라인으로 렌더링됩니다.

For security, this is not a client-side widget config flag. It is a server-side setting, validated when each comment is saved, so it cannot be turned on from the page. Only iframes pointing at a built-in list of trusted providers are allowed. Any other iframe is removed.

보안을 위해, 이는 클라이언트 측 위젯 구성 플래그가 아닙니다. 각 댓글이 저장될 때 검증되는 서버 측 설정이며, 페이지에서 켤 수 없습니다. 신뢰할 수 있는 제공업체의 내장 목록을 가리키는 iframe만 허용됩니다. 그 외의 모든 iframe은 제거됩니다.

This is done without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='위젯 사용자 정의 페이지에서 미디어 임베드 설정이 켜져 있어, 댓글 작성자가 신뢰할 수 있는 iframe 임베드를 붙여넣을 수 있습니다.'; title='미디어 임베드 허용' app-screenshot-end]

### 자체 제공업체 추가

If you want to allow embeds from a provider that is not on the built-in trusted list, add its hostname in the "Additional Embed Domains" field on the same page. These hostnames are allowed in addition to the built-in providers. Matching is exact, so include the full hostname (for example, player.example.com). Anything you do not list stays blocked.

내장된 신뢰 목록에 없는 제공업체의 임베드를 허용하려면, 같은 페이지의 "Additional Embed Domains" 필드에 해당 호스트명을 추가하십시오. 이러한 호스트명은 내장된 제공업체에 추가로 허용됩니다. 일치 여부는 정확히 비교되므로 전체 호스트명(예: player.example.com)을 포함해야 합니다. 목록에 포함되지 않은 모든 항목은 차단됩니다.

Both the plain comment box and the WYSIWYG editor support pasting an embed. In the WYSIWYG editor the embed is inserted as a removable block.

일반 댓글 입력 상자와 WYSIWYG 편집기 모두 임베드 붙여넣기를 지원합니다. WYSIWYG 편집기에서는 임베드가 제거 가능한 블록으로 삽입됩니다.