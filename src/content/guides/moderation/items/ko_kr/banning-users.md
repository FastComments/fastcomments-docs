FastComments를 사용하여 사이트에서 사용자가 댓글을 달지 못하도록 차단하는 방법은 두 가지가 있습니다.

첫 번째 방법은 이미 이메일을 알고 있는 경우, <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">차단된 사용자</a> 페이지에 입력하는 것입니다.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Moderate Comments 아래에 있는 차단된 사용자 목록으로, 차단된 이메일 주소와 새 차단을 추가하는 버튼이 포함됩니다.'; title='차단된 사용자 페이지' app-screenshot-end]

이 페이지는 Moderate Comments -> Banned Users 경로를 통해 접근할 수 있습니다.

사용자를 차단하려면 차단 유형을 선택할 수 있으며, 영구 차단 또는 영구 섀도우 차단 중 하나를 선택합니다:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='이메일 필드와 영구 차단 또는 영구 섀도우 차단 중 선택할 수 있는 차단 유형 옵션이 있는 새 차단 양식'; title='사용자 차단' app-screenshot-end]

두 번째 방법은 Comment Moderation 페이지의 각 댓글에 배치된 차단 버튼을 클릭하는 것입니다.

차단 버튼을 클릭하면 몇 가지 옵션이 표시되며, 차단 유형과 기간을 지정할 수 있습니다.

### 이메일 별칭

이메일로 사용자를 차단할 때 FastComments는 `+` 별칭을 자동으로 무시합니다. 예를 들어, `user+alias@gmail.com`을 차단하면 `user@gmail.com` 및 해당 주소의 다른 `+` 변형(예: `user+other@gmail.com`)도 차단됩니다.

### 섀도우 차단

섀도우 차단은 사용자의 댓글이나 투표가 성공적으로 저장된 것처럼 보이게 하는 차단 유형으로, 실제로는 저장되지 않았습니다. 특정 상황에서 이것이 바람직할 수 있습니다.

### IP 주소를 통한 차단

테넌트가 선택적으로 제외하지 않는 한, FastComments는 댓글 작성자의 IP 주소를 해시된 형태로 저장하여 IP 기반 차단을 지원합니다.

### 차단된 사용자 검색

목록이 한두 페이지 이상으로 늘어나면, 테이블 위의 검색 행을 사용하여 목록을 좁힐 수 있습니다.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Banned Users 페이지의 검색 행으로, Search By 드롭다운, Match 드롭다운 및 Value 입력 필드가 포함됩니다.'; title='차단된 사용자 검색' app-screenshot-end]

세 가지 제어 요소가 있습니다:

- **Search By**는 검색할 필드를 선택합니다: Any Field, Email, Name, Banned By, 또는 Banned For Saying. 마지막 네 항목은 테이블의 동일한 이름을 가진 열에 해당합니다.
- **Match**는 비교 방식을 선택합니다. **Contains**는 값이 필드 어디에든 포함되는 경우를 찾고, **Equals**는 전체 필드와 일치하는 경우를 찾습니다.
- **Value**는 검색할 텍스트입니다.

모든 필드는 대소문자를 구분하지 않고 일치하므로, `SPAMMER@EXAMPLE.COM`을 검색하면 `spammer@example.com`으로 저장된 차단을 찾게 됩니다.

알아두면 좋은 몇 가지 사항:

- **Banned For Saying**은 사용자를 차단하게 만든 댓글의 텍스트를 검색합니다. 특정 문구로 차단된 모든 사용자를 찾는 방법입니다.
- **Banned By**는 차단을 실행한 중재자의 이름을 검색합니다. 이는 다른 중재자의 결정을 검토할 때 유용합니다.
- 와일드카드 차단은 `*`와 함께 저장되므로, `bademail.com`에 대한 **Contains** 검색은 `*@bademail.com` 차단을 찾습니다.
- **Name**은 Name 열에 표시된 이름과 일치하므로, 차단된 이후 이름을 변경했더라도, 이메일 주소만 입력해 차단을 생성했을 때 이름이 기록되지 않았더라도 사용자를 찾을 수 있습니다. 차단에 기록된 이름도 일치하므로, 이전 이름이나 현재 이름 중 어느 것이든 검색하면 작동합니다.
- **Any Field**는 이메일, 이름, 차단한 중재자, 차단된 댓글 텍스트를 모두 함께 검색합니다.

검색은 페이지 URL의 일부이므로, 다른 중재자와 필터링된 목록을 다른 모더레이션 링크와 동일하게 공유할 수 있습니다. 결과 페이지를 이동해도 검색이 유지되며, 새 검색을 시작하면 첫 페이지로 돌아가고, **Clear**를 클릭하면 전체 목록으로 돌아갑니다.