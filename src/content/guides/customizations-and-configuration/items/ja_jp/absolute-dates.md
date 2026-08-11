[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

デフォルトでは、ローカライズされた相対日時が使用されます。例えば、最近投稿されたコメントの横に「11 分前」と表示されることがあります。

絶対日時を使用する必要がある、または使用したい場合は、このパラメータを true に設定します。 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = '絶対日時を使用する'; code-example-end]

コードを使用せずに、ウィジェットのカスタマイズページの「詳細オプション」セクションでカスタマイズできます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='ウィジェットカスタマイズページの詳細オプションで、絶対日時トグルがオンになっている状態'; title='絶対日時を使用する' app-screenshot-end]