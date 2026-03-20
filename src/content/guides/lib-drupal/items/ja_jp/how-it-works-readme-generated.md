---
ユーザーが FastComments フィールドが有効なエンティティを訪問すると:

1. FastComments の JavaScript ウィジェットが CDN から読み込まれます.
2. SSO が構成されている場合、ユーザーの Drupal の識別情報が FastComments に渡されます.
3. `<noscript>` フォールバックは、JavaScript を使用していないユーザー向けにサーバー側でレンダリングされたコメントを提供します (Live Comments および Streaming Chat モードのみ).
---