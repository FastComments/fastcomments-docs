With FastComments SSO Access Control, sometimes referred to as RBAC, users can be restricted to only access certain pages, or comment threads. Additionally,
사용자는 동일한 그룹에 있는 사용자만 `@mention`할 수 있습니다.

## 자세히

우리는 `Users`와 선택적으로 `Pages`를 그룹에 배치할 수 있습니다.

`Users`가 그룹에 배치되고 `Limit Comments by SSO User Groups`가 위젯 설정에서 활성화되면, 그런 경우 사용자들은
동일한 그룹에 속한 사용자들의 댓글만 보게 되며 동일한 그룹의 사용자들만 `@mention`할 수 있습니다.

또한, `Pages`를 그룹에 배치할 수 있으며, 그 경우 사용자는 접근 권한이 있는 페이지의 댓글만 볼 수 있습니다.

이를 "User-Level" 그룹과 "Page-Level" 그룹으로 구분합니다.

그렇다면 어느 것이 적합할까요?

#### User-Level 그룹을 사용할 경우...

- 같은 `urlId` 값(페이지 URL 또는 기사 ID)을 사용하되, 그룹별로 댓글을 제한하려는 경우.
- 예를 들어, "New User"와 "Veteran User" 그룹을 만들고 동일한 페이지에서 서로의 댓글을 절대 보지 못하게 하려는 경우.

#### Page-Level 그룹을 사용할 경우...

- 그룹마다 특정 페이지가 있는 경우.
- 예를 들어, "Public Pages" 그룹에 속한 사용자는 "Top Secret" 기사들을 절대 볼 수 없어야 합니다.