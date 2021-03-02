[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

With Simple SSO, we can provide the commenting widget with information about the user so that they don't have to enter their username or email to comment.

We can configure Simple SSO as follows:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatarSrc: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob" }}; linesToHighlight = [6, 7, 8, 9, 10, 11]; title = 'Simple SSO'; code-example-end]

The user won't actually be logged in, but it will appear as so in the commenting widget. When they take any action, we will email them asking to verify it, if an
email is provided.

Note that providing an email with Simple SSO is not required, however by default their comments will show as "Unverified".

If the provided username is not unique, they will be asked for a new username when attempting to comment.
