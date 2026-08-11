[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

По умолчанию FastComments будет сортировать комментарии по направлению сортировки «Самая релевантная».

Сортировка «Самая релевантная» учитывает время оставления комментария и количество голосов при сортировке.

Пользователь может затем изменить направление сортировки на «Старейшие сначала» или «Новейшие сначала» в пользовательском интерфейсе виджета комментариев.

Тем не менее, мы можем изменить значение по умолчанию на любое из трёх. Например, если вы хотите показывать старейшие комментарии первыми:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Мы устанавливаем значение **defaultSortDirection** в "OF", чтобы задать направление «OF».

Для направления сортировки «Новейшие сначала» мы бы сделали следующее:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Допустимые значения для **defaultSortDirection**:

- MR: "Самый недавний"
- NF: "Новейшие сначала"
- OF: "Старейшие сначала"

Это также можно сделать без кода. На странице настройки виджета см. раздел "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Селектор направления сортировки по умолчанию, предлагающий «Самую релевантную», «Новейшие сначала» и «Старейшие сначала»'; title='Изменение направления сортировки по умолчанию' app-screenshot-end]

Обратите внимание, что комментарии на каждой странице для каждого направления сортировки предварительно вычисляются, поэтому все направления сортировки имеют одинаковую производительность.