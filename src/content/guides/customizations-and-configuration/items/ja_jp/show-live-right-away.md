[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

デフォルトでは、ライブコメント機能が有効になっています。つまり、コメントが追加、削除、編集、ピン留めされた場合、その変更は同じコメントスレッドを閲覧しているすべてのユーザーにリアルタイムで表示されます。

ただし、デフォルトでは、これらの新着コメントは「Show 2 New Comments」のようなテキストの動的に表示されるボタンの下に表示されます。

新着コメントがページへの直接の返信である場合、ボタンはコメントスレッドの上部に表示されます。特定のコメントへの返信である場合は、そのコメントの下に表示されます。

これは、ページのサイズが絶えず変化してユーザーがスクロールバーを掴もうとする際に不満を感じる可能性があることを防ぐためです。

ライブオークションやオンラインイベントなどの一部のユースケースでは、これは望ましい動作ではありません。コメントウィジェットを、新着コメントが「すぐに表示される」チャットボックスのようなものにしたい場合があります。

そのため、この機能を有効にするフラグの名前は **showLiveRightAway** となっています。

以下のようにして有効にできます:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = '新着コメントをすぐに表示する'; code-example-end]

コードを使わずに、ウィジェットカスタマイズページでこれを設定できます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='ライブコメントを折りたたむ設定が切り替えられ、ボタンの背後ではなく新着コメントが即座に表示される'; title='新着コメントをすぐに表示する' app-screenshot-end]