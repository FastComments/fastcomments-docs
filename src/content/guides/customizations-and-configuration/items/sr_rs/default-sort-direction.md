[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

По подразумевању, FastComments ће сортирати коментаре по смеру сортирања „Most Relevant“.

Сортирање „Most Relevant“ узима у обзир време када је коментар остављен и број гласова приликом сортирања.

Корисник потом може у UI видгета за коментаре да промени смер сортирања на „Најстарије прво“ или „Најновије прво“.

Међутим, можемо променити подразумевано на било који од ова три. На пример, ако желите да прикажете најстарије коментаре прво:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Подешавамо вредност **defaultSortDirection** на "OF" да бисмо поставили смер на "OF".

За смер сортирања „Најновије прво“, урадили бисмо следеће:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Валидне вредности за **defaultSortDirection** су:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Ово се може урадити и без кода. На страници за прилагођавање видгета, погледајте одељак „Подразумевани смер сортирања“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Имајте у виду да се коментари на свакој страници за сваки смер сортирања предрачунавају, тако да сви смерови сортирања имају исте перформансе.