REST API를 통해 여러분의 코드에서도 투표를 관리할 수 있습니다. 예를 들어 백엔드에서 댓글에 투표를 추가하거나, 투표를 종료하거나, 투표 결과를 내보낼 수 있습니다:

- [Poll Structure](/guide-api.html#poll-structure)는 투표 객체와 그 제한 사항을 설명합니다.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId`는 댓글의 투표를 읽기, 교체, 수정 및 제거합니다.
- [PollVote Structure](/guide-api.html#poll-vote-structure)는 개별 투표를 설명합니다.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes`는 투표를 조회, 제출 및 제거합니다.

`POST /api/v1/comments`에 `poll`을 전달하여 댓글과 함께 투표를 생성할 수도 있습니다.

API는 위젯과 동일한 설정을 따릅니다: 투표가 활성화되어 있어야 하며, 익명 투표가 허용되지 않는 한 로그인하지 않은 사용자의 투표는 거부됩니다. 익명 투표의 개별 투표 내역은 API를 통해 읽을 수 없습니다.