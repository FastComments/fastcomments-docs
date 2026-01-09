[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

По умолчанию FastComments сортирует комментарии по направлению сортировки "Most Relevant".

Сортировка "Most Relevant" учитывает время оставления комментария и количество голосов при определении порядка.

Пользователь затем может изменить направление сортировки на «Старые сначала» или «Новые сначала» в интерфейсе виджета комментариев.

Однако мы можем изменить значение по умолчанию на любое из трёх. Например, если вы хотите показывать сначала самые старые комментарии:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Мы устанавливаем значение **defaultSortDirection** в "OF", чтобы задать направление "OF".

Для сортировки с сначала новыми комментариями, мы сделали бы следующее:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Допустимые значения для **defaultSortDirection**:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Это также можно сделать без кода. На странице настройки виджета см. раздел "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Обратите внимание, что комментарии на каждой странице для каждого направления сортировки предвычисляются, поэтому производительность одинакова для всех направлений сортировки.