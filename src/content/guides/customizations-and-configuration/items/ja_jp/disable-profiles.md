[related-parameter-start name = 'disableProfiles'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザーのアバターをクリックするとそのユーザーのプロファイルを表示します。

ただし、この機能は無効にすることができます:

[code-example-start config = {disableProfiles: true}; linesToHighlight = [6]; title = 'プロファイルの無効化'; code-example-end]

コードを使用せずにこの設定を行うこともできます。ウィジェットのカスタマイズページで「プロファイルの無効化」セクションを確認してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profiles']; selector = '.disable-profiles'; alt='ウィジェットカスタマイズページで「プロファイルの無効化」チェックボックスがオンになっているため、アバターをクリックしてもプロファイルが開かなくなります'; title='プロファイルの無効化' app-screenshot-end]

---