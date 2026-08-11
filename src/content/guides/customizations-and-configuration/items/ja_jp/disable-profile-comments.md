[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザープロフィールに「Profile Comments」タブを表示し、訪問者が誰かのプロフィールにコメントを残すことができます。

ただし、このタブを無効にすることができます:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Disable Profile Comments'; code-example-end]

コードを使用せずにもこれを行うことができます。ウィジェットカスタマイズページで「Disable Profile Comments」セクションをご覧ください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='ウィジェットカスタマイズページで、プロフィールコメントタブを非表示にするために「Disable Profile Comments」チェックボックスがチェックされた状態'; title='プロフィールコメントの無効化' app-screenshot-end]