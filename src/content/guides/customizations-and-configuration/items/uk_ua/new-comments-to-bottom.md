[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

За замовчуванням нові live-коментарі з'являються у верхній частині списку коментарів у міру їхньої публікації в реальному часі.

Коли ця опція увімкнена, нові live-коментарі замість цього додаватимуться внизу списку. Це впливає на те, як коментарі з'являються під час їхньої публікації в реальному часі, коли користувачі переглядають тему коментарів.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

With this setting enabled:
- New live comments posted by other users will appear at the bottom of the comment list
- Users will see new comments appear below existing comments in real-time
- This only affects live comment updates - not the initial page load
- This can help maintain reading flow when users are following a discussion

Note that this setting only affects where new live comments are placed as they arrive in real-time. It does not affect the initial sort order when the page loads.