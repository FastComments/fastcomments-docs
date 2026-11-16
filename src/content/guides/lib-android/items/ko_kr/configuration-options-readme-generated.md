SDK는 `CommentWidgetConfig` 클래스를 통해 다양한 구성 옵션을 제공합니다:

| 옵션 | 설명 |
|--------|-------------|
| `tenantId` | FastComments 계정 ID |
| `urlId` | 현재 페이지를 나타내는 ID |
| `sso` | 인증용 SSO 토큰 |
| `allowAnon` | 익명 댓글 작성 허용 |
| `voteStyle` | UpDown 또는 Heart 투표 스타일 |
| `hideAvatars` | 사용자 아바타 숨기기 |
| `hasDarkBackground` | 다크 모드 여부 표시 |
| `customCSS` | 사용자 정의 CSS 스타일 |
| `enableInfiniteScrolling` | 무한 스크롤 페이지네이션 활성화 |
| `readonly` | 댓글 작성은 비활성화하되 댓글은 표시 |
| `disableVoting` | 투표 기능 비활성화 |
| `disableLiveCommenting` | 실시간 업데이트 비활성화 |
| `pollCreation` | 댓글에 투표를 첨부할 수 있는 대상: `NUMBER_0` 비활성화, `NUMBER_1` 관리자 및 중재자, `NUMBER_2` 모든 사용자 |
| `pollVoting` | 투표에 참여할 수 있는 대상: `NUMBER_0` 로그인한 사용자, `NUMBER_1` 모든 사용자 |