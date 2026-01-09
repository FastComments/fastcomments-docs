[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

이 목록은 [모더레이션 그룹](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) 페이지에서 생성된 ID들의 목록입니다.

지정하면, 해당 설정을 사용하여 남긴 댓글에는 동일한 `moderationGroupIds` 세트가 포함됩니다.

만약 `Moderator`가 하나 이상의 [모더레이션 그룹](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) 을 정의해 둔 경우, 그들은
자신의 그룹과 연관된 댓글만 `Moderate Comments` 페이지에서 보게 됩니다.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]