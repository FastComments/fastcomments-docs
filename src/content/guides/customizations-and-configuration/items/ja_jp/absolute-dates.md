[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

デフォルトでは、ローカライズされた相対日時が使用されます。たとえば、最近投稿されたコメントの横に "11分前" と表示されることがあります。

絶対日時を使用する必要がある、または望ましい場合は、このパラメータを true に設定します。 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

これはコードを使わず、ウィジェットのカスタマイズページの「詳細オプション」から設定できます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]