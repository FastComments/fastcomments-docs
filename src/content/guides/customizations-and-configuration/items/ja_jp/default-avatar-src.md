[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

ユーザーが FastComments で初めてコメントすると、<a href="https://gravatar.com/" target="_blank">http://gravatar.com/</a> からアバターを取得しようとします。

ただし、アバターが見つからない場合や、ユーザーがアカウントでアバターを設定していない場合は、静的なデフォルトアバター画像を表示します。

独自の静的アバター画像を指定するには、*defaultAvatarSrc* 設定を使用します。

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

コードを使用せずに行うこともできます。ウィジェットカスタマイズページで「Default Avatar」セクションをご覧ください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='ウィジェットカスタマイズページのデフォルトアバターセクションで、フォールバックアバター画像の URL を設定します'; title='デフォルトアバターのカスタマイズ' app-screenshot-end]

SSO などで特定のユーザーのアバターを定義する方法は、別のセクションで説明しています。