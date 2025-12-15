[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

A list of ids generated from the [Moderation Groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) page.

When specified, comments left using the specified configuration will contain the same set of `moderationGroupIds`.

If a `Moderator` has one or more [Moderation Groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) defined, they will
only see comments in the `Moderate Comments` page associated with their group(s).

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]
