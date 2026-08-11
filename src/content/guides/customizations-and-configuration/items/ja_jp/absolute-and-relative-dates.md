[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

デフォルトでは、ローカライズされた相対日時が使用されます。例えば、最近投稿されたコメントの横に「11 分前」と表示されることがあります。

この相対日時形式を保持しつつ、完全な日付も併せて表示したい、または必要な場合は、このパラメータを true に設定します。 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = '絶対日時と相対日時の両方を使用'; code-example-end]

この設定はコードを書かずに、ウィジェットカスタマイズページの「高度なオプション」セクションでカスタマイズできます。まず、UI でこのオプションを表示させるために「絶対日時」を有効にする必要があります。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='ウィジェットカスタマイズページの高度なオプションで、絶対日時と相対日時の組み合わせ設定が有効になっている状態'; title='絶対日時と相対日時の両方を使用' app-screenshot-end]

---