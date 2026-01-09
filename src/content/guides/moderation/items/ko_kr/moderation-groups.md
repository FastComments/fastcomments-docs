---
모더레이터는 서로 다른 페이지나 콘텐츠 카테고리를 중재하도록 그룹에 배치할 수 있습니다.

모더레이터가 하나 이상의 그룹에 속해 있으면, 댓글 관리 페이지에서 해당 그룹의 댓글만 보게 됩니다.

예를 들어, 카테고리별로 비디오를 보여주는 사이트를 운영한다고 합시다. 고양이, 개, 앵무새 비디오마다 다른 모더레이터를 두고 싶을 수 있으므로, [그 그룹들을 추가해 보겠습니다](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

모더레이터를 추가하면, 그 모더레이터가 속할 하나 이상의 그룹을 선택할 수 있는 옵션이 생깁니다:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

마지막으로, 해당 모더레이터가 댓글을 볼 수 있도록 댓글을 하나 이상의 그룹에 연결해야 합니다.

이 설정은 [그룹 몇 개를 추가](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)한 다음, 댓글 위젯에서 해당 `Moderation Group` ids를 지정하면 됩니다,
[여기에서 안내된 대로](/guide-customizations-and-configuration.html#moderation-group-ids).

---