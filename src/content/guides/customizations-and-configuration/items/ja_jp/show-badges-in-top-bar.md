[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザーのバッジをコメントスレッド内のコメントにのみ表示します。

ただし、ウィジェットのカスタマイズページでこの機能を有効にすると、コメントフォームの上にある名前の隣にユーザーのバッジを表示できます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

これにより、ユーザーがコメントを作成しているときに、トップバー領域で名前の横にバッジが表示され、実績やステータスがより目立つようになります。

この機能が動作するには、ウィジェットのカスタマイズUIで有効にする必要がある点に注意してください。サーバー側で有効になっている場合でも、コード構成で **showBadgesInTopBar** フラグを false に設定して、個別に無効化することもできます:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]