[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Por padrão, datas relativas localizadas são usadas. Por exemplo, ao lado de um comentário recém‑postado, você pode ver "há 11 minutos".

Pode ser necessário ou desejado manter esse formato de data relativa, mas também exibir a data completa ao lado, caso em que você define este parâmetro como true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget, em Opções avançadas. Primeiro, você precisará habilitar Datas Absolutas para ver esta opção na interface.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Opções avançadas na página de personalização do widget com datas absolutas e a configuração combinada de data relativa ativada'; title='Usar Datas Absolutas e Relativas' app-screenshot-end]