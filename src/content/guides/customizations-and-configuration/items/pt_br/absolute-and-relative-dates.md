[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Por padrão, são usadas datas relativas localizadas. Por exemplo, ao lado de um comentário deixado recentemente você pode ver "há 11 minutos".

Pode ser necessário ou desejável manter esse formato de data relativa, mas também exibir a data completa ao lado dele, caso em que você define este parâmetro como true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Isso pode ser customizado sem código, na página de personalização do widget, em Opções Avançadas. Primeiro você terá que ativar Absolute Dates para ver esta opção na UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---