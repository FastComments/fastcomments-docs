[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

デフォルトでは、ライブコメントが有効になっています。これは、コメントが追加、削除、編集、またはピン留めされた場合、変更がコメントスレッドを閲覧しているすべてのユーザーに同時に表示されることを意味します。

しかし、デフォルトでは新しいコメントは「Show 2 New Comments」のようなテキストが表示された動的なボタンの下に表示されます。

新しいコメントがページへの直接の返信である場合、ボタンはコメントスレッドの上部に表示されます。特定のコメントへの返信である場合、ボタンはそのコメントの下に表示されます。

これは、ページサイズがユーザー側で常に変化するのを防ぎ、スクロールバーを掴もうとしたときのフラストレーションを防ぐためです。

ライブ入札やオンラインイベントなどの一部のユースケースでは、この動作は望ましくありません。コメントウィジェットを「チャット」ボックスのように、新しいコメントが「すぐに表示」されるようにしたい場合があります。

したがって、その機能を有効にするフラグの名前は **showLiveRightAway** です。

We can turn it on as follows:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'ライブコメントをすぐに表示'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='ライブコメントの折りたたみ設定が切り替わり、新しいコメントがボタンの背後ではなく即座に表示されるようになります'; title='ライブコメントをすぐに表示' app-screenshot-end]