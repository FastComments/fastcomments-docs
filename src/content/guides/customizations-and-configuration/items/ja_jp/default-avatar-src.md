[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

ユーザーが FastComments で初めてコメントする際、<a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a> からアバターを取得しようとします。

しかし、アバターが見つからない場合や、ユーザーがアカウントにアバターを設定していない場合は、静的なデフォルトのアバター画像を表示します。

独自の静的アバター画像を指定するには、*defaultAvatarSrc* 設定を使用できます。

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

これはコードなしでも行えます。ウィジェットのカスタマイズページで「Default Avatar」セクションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

特定のユーザーのアバターを定義する（SSO を使用する場合など）については、別のセクションで説明しています。

---