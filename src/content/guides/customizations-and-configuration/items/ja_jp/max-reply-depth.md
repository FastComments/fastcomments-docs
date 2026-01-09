---
[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

デフォルトでは、FastComments は返信のネストを無制限に許可し、ユーザーが返信に対して際限なく返信できるスレッド構造を作成します。

maxReplyDepth オプションを使うと、返信スレッドの深さを制限できます。最大深度に達すると、そのレベルのコメントには返信ボタンが表示されなくなります。

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

With maxReplyDepth set to 2:
- ユーザーはトップレベルでコメントできます（depth 0）
- ユーザーはトップレベルのコメントに返信できます（depth 1）
- その返信にさらに返信できます（depth 2）
- depth 2 を超える返信は許可されません

maxReplyDepth を 1 に設定すると、トップレベルのコメントへの返信のみが許可され、より平坦な議論の構造になります。

maxReplyDepth を 0 に設定すると、すべての返信が無効になり、トップレベルのコメントのみが許可されます。指定しない場合、返信は無制限にネストされます。
---