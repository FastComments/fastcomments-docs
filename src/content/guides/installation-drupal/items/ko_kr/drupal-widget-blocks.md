이 모듈은 `Structure > Block layout` (`/admin/structure/block`)에서 배치할 수 있는 여러 블록을 제공합니다.

- **FastComments Widget** - 기본 댓글 위젯입니다. 현재 엔티티를 자동으로 감지합니다. FastComments 필드가 이미 첨부된 엔티티는 건너뛰므로 동일한 페이지에 중복 위젯이 표시되지 않습니다.
- **FastComments Live Chat** - 실시간 스트리밍 채팅입니다. 동일한 페이지에서 댓글 필드 옆에 배치할 수 있습니다.
- **FastComments Collab Chat** - 텍스트 선택 주석 및 토론 기능입니다.
- **FastComments Image Chat** - 이미지에서 좌표 기반 주석입니다. 방문자는 이미지의 특정 위치를 클릭하여 해당 위치에 연결된 댓글을 남깁니다.
- **FastComments Recent Comments** - 사이트 전반의 최신 댓글을 표시합니다. 개수는 블록에서 구성할 수 있습니다.
- **FastComments Top Pages** - 사이트에서 가장 댓글이 많은 페이지를 표시합니다.

콘텐츠 중심 블록(Live Chat, Collab Chat, Image Chat)은 현재 엔티티를 자동으로 감지하며, 엔티티가 아닌 페이지에서는 경로 기반 식별자로 대체됩니다. 즉, 추가 설정 없이 taxonomy 페이지, views, 커스텀 라우트에서 작동합니다.