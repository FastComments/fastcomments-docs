[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

By default, FastComments will render the comment input box and comment thread at the same time. To save some vertical space,
세로 공간을 절약하기 위해, it will also hide any other required fields until the widget is interacted with.
위젯과 상호작용할 때까지 다른 필수 필드들도 숨깁니다.

However, the comment widget can be hidden behind a button, for example:
그러나 댓글 위젯은 예를 들어 버튼 뒤에 숨길 수 있습니다:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

The button uses different translated text based on whether the comments are currently shown or not. If the comments are hidden, it uses `translations.SHOW_COMMENTS_BUTTON_TEXT`. If the
버튼은 댓글이 현재 표시되어 있는지 여부에 따라 서로 다른 번역된 텍스트를 사용합니다. 댓글이 숨겨져 있으면 `translations.SHOW_COMMENTS_BUTTON_TEXT`를 사용합니다. 만약
comments are shown, it uses `translations.HIDE_COMMENTS_BUTTON_TEXT`. The translations can contain the text `[count]` which will
댓글이 표시되어 있으면 `translations.HIDE_COMMENTS_BUTTON_TEXT`를 사용합니다. 번역 문자열은 `[count]` 텍스트를 포함할 수 있으며
be replaced by the localized count.
이는 지역화된 카운트로 대체됩니다.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

This is designed to replace the `hideCommentsUnderCountTextFormat` configuration.
이 기능은 `hideCommentsUnderCountTextFormat` 구성 옵션을 대체하도록 설계되었습니다.

The count is updated live with the comment thread. The button is not shown if there are no comments.
카운트는 댓글 스레드와 실시간으로 업데이트됩니다. 댓글이 없으면 버튼은 표시되지 않습니다.

This can be enabled without code by creating a customization rule and enabling "Click to Show Comments":
이 기능은 커스터마이제이션 규칙을 생성하고 "클릭하여 댓글 표시"를 활성화하면 코드 없이 사용 설정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]