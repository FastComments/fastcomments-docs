[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Por padrão, o FastComments ordenará os comentários pela direção de classificação "Mais Relevante".

A ordenação Mais Relevante leva em conta o horário em que o comentário foi deixado e o número de votos para a classificação.

O usuário pode então mudar a direção de classificação para Mais Antigos ou Mais Recentes Primeiro na interface do widget de comentários.

No entanto, podemos alterar o padrão para qualquer um dos três. Por exemplo, se você quiser mostrar os comentários mais antigos primeiro:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Alterando a classificação padrão para Mais Antigos Primeiro'; code-example-end]

Definimos o valor de **defaultSortDirection** como "OF" para definir a direção como "OF".

Para a direção de classificação mais recentes primeiro, faríamos o seguinte:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Alterando a classificação padrão para Mais Recentes Primeiro'; code-example-end]

Os valores válidos para **defaultSortDirection** são:

- MR: "Mais Recente"
- NF: "Mais Recentes Primeiro"
- OF: "Mais Antigos Primeiro"

Isso também pode ser feito sem código. Na página de personalização do widget, veja a seção "Direção de Classificação Padrão".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Seletor de Direção de Classificação Padrão oferecendo Mais Relevante, Mais Recentes Primeiro e Mais Antigos Primeiro'; title='Alterando a Direção de Classificação Padrão' app-screenshot-end]

Observe que os comentários em cada página para cada direção de classificação são pré-calculados, portanto todas as direções de classificação têm o mesmo desempenho.

---