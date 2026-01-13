[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

По умолчанию FastComments будет сортировать комментарии по направлению сортировки "Most Relevant".

Сортировка "Most Relevant" учитывает время, когда был оставлен комментарий, и количество голосов при сортировке.

Пользователь затем может изменить направление сортировки на либо Oldest, либо Newest First в пользовательском интерфейсе виджета комментариев.

Однако мы можем изменить значение по умолчанию на любое из трёх. Например, если вы хотите показывать сначала самые старые комментарии:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Мы устанавливаем значение **defaultSortDirection** в "OF", чтобы задать направление "OF".

Для направления сортировки "новые сначала" мы бы сделали следующее:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Допустимые значения для **defaultSortDirection**:

- MR: "Самые недавние"
- NF: "Сначала новые"
- OF: "Сначала самые старые"

Это также можно сделать без кода. На странице настройки виджета см. раздел "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Обратите внимание, что комментарии на каждой странице для каждого направления сортировки предварительно вычисляются, поэтому все направления сортировки имеют одинаковую производительность.