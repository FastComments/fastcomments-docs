## Actions and Searches

동작은 FastComments에 데이터를 생성하고, 검색은 데이터를 조회하여 이후 단계에서 사용할 수 있게 합니다. 각 동작은 FastComments REST API를 호출하며, 호출에 필요한 API 크레딧을 사용합니다: 별도 언급이 없는 한 호출당 1 크레딧이 소모됩니다.

## Create Comment

페이지에 댓글을 게시합니다.

| Field | Required | Notes |
|-------|----------|-------|
| Page URL ID | Yes | 페이지 위젯이 페이지에서 사용하는 URL ID입니다. 댓글은 이 ID로 그룹화됩니다. |
| Page URL | Yes | 알림 이메일에 사용되는 전체 페이지 URL입니다. |
| Comment | Yes | FastComments 마크다운 형식의 댓글 본문입니다. |
| Commenter Name | Yes | 이름은 이메일당 고유하므로, 다른 이메일로 같은 이름을 재사용하면 실패합니다. |
| Commenter Email | No | 아직 존재하지 않는 경우 해당 이메일에 대한 사용자가 생성됩니다. |
| User ID | No | 기존 SSO 사용자 ID입니다. 이름 및 이메일보다 우선합니다. |
| Parent Comment ID | No | 답글을 게시하려면 설정합니다. |
| Approved, Verified | No | 기본값은 모두 true입니다. 승인되지 않은 댓글은 검토될 때까지 숨겨집니다. |
| Posted At | No | 기본값은 현재 시각입니다. |
| Avatar URL, Page Title, Locale | No | 로케일 기본값은 `en_us`입니다. |
| Show Live In Widget | No | 댓글을 실시간으로 시청자에게 푸시합니다. 1크레딧 대신 2크레딧이 소모됩니다. |
| Run Spam Check, Send Emails | No | 기본적으로 비활성화됩니다. |

## Create Page

댓글이 존재하기 전에 페이지 레코드를 생성하여 목록에 표시하고 제한할 수 있게 합니다. URL ID, 제목, URL을 입력받으며, 선택적으로 해당 페이지를 볼 수 있는 SSO 그룹 ID를 지정할 수 있습니다.

## Create SSO User

싱글 사인온 사용자를 생성합니다. 자체 사용자 ID, 사용자명 및 이메일을 입력받으며, 선택적으로 표시 이름, 표시 라벨, 아바타, 웹사이트, 그룹 ID, 알림 및 개인정보 플래그를 지정할 수 있습니다. 관리 역할은 Zapier를 통해 부여할 수 없습니다.

## Create Feed Post

HTML 콘텐츠를 사용하여 FastComments 피드에 포스트를 생성합니다. 작성자 사용자 ID가 필요합니다(FastComments 또는 SSO 사용자 ID); 제목, 태그 및 하나의 링크 미리보기는 선택 사항입니다.

## Create Hash Tag

댓글 작성자가 사용할 수 있는 해시 태그를 생성하며, 선택적으로 연결될 URL을 지정할 수 있습니다. 태그는 계정당 고유하므로, 매 실행마다 태그를 생성하는 Zap은 고유한 값을 포함해야 합니다.

## Flag Comment

댓글을 검토를 위해 플래그 지정합니다. 플래그를 지정하는 사용자의 ID가 필요합니다; Create Comment에서 반환된 작성자 ID를 사용할 수 있습니다.

## Searches

| Search | Input | Returns |
|--------|-------|---------|
| Find Comment | Comment ID | 댓글이 있으면 반환, 없으면 없음. |
| Find SSO User | Email | SSO 사용자가 있으면 반환, 없으면 없음. |
| Find Page | URL ID | 페이지가 있으면 반환, 없으면 없음. |

검색 결과가 없더라도 Zap이 실패하지 않습니다. Zapier의 "find or create" 모드에서 검색과 생성을 결합하면 페이지나 사용자가 없을 경우 자동으로 생성할 수 있습니다.