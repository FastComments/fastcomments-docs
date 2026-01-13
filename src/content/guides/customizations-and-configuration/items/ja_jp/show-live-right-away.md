[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

デフォルトでは、ライブコメント機能は有効になっています。つまり、コメントが追加、削除、編集、またはピン留めされると、変更は表示される
すべてのユーザーに同時に反映されます。

ただし、デフォルトではそれらの新しいコメントは「Show 2 New Comments」のような文言が表示された動的なボタンの下に表示されます。

新しいコメントがページに直接対する返信である場合、そのボタンはコメントスレッドの上部に表示されます。特定のコメントへの返信である場合は、
そのコメントの下にボタンが表示されます。

これは、ページのサイズがユーザーに対して絶えず変化し、スクロールバーを掴もうとしたときにフラストレーションを引き起こす可能性があるのを防ぐためです。

ライブ入札やオンラインイベントのようなユースケースでは、これは望ましくない動作であることがあります — コメントウィジェットが新しいコメントを「すぐに表示する」チャットボックスのようになってほしい場合があります。

したがって、その機能を有効にするフラグの名前は次のとおりです: **showLiveRightAway**。

次のようにオンにできます:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

コードを使わず、ウィジェットのカスタマイズページでこれをカスタマイズすることもできます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---