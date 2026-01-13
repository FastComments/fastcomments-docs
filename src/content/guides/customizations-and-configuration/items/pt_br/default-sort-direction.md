[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Por padrão, o FastComments ordenará os comentários pela direção de ordenação "Most Relevant".

A ordenação "Most Relevant" leva em conta o horário em que o comentário foi deixado e o número de votos para fins de ordenação.

O usuário pode então alterar a direção de ordenação para "Antigos primeiro" ou "Novos primeiro" na interface do widget de comentários.

No entanto, podemos alterar o padrão para qualquer um dos três. Por exemplo, se você quiser mostrar os comentários mais antigos primeiro:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Definimos o valor de **defaultSortDirection** para "OF" para definir a direção para "OF".

Para a direção de ordenação "Novos primeiro", faríamos o seguinte:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Os valores válidos para **defaultSortDirection** são:

- MR: "Mais recentes"
- NF: "Novos primeiro"
- OF: "Antigos primeiro"

Isso também pode ser feito sem código. Na página de customização do widget, veja a seção "Direção de ordenação padrão".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Observe que os comentários em cada página para cada direção de ordenação são pré-computados, portanto todas as direções de ordenação têm o mesmo desempenho.