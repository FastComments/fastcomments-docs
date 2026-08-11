[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はコメントスレッド内のコメントに対してのみユーザーバッジを表示します。

ただし、ウィジェットカスタマイズページでこの機能を有効にすることで、コメントフォーム上部の名前の横にユーザーバッジを表示できます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='ウィジェットカスタマイズページのトップバーにバッジを表示するチェックボックスで、コメントフォーム上部の名前の横にバッジを配置します'; title='トップバーにバッジを表示するオプション' app-screenshot-end]

これにより、トップバー領域でユーザーの名前の横にバッジが表示され、コメント作成時にユーザーの実績やステータスがより目立つようになります。

この機能が動作するには、ウィジェットカスタマイズ UI で有効にする必要があることに注意してください。サーバーレベルで有効になっている場合でも、コード設定で **showBadgesInTopBar** フラグを false に設定して個別に無効化することができます。

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'トップバーのバッジ表示を無効にする'; code-example-end]