[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

По подразумеваном, FastComments ће сортирати коментаре по смеру сортирања „Most Relevant“.

Сортирање „Most Relevant“ узима у обзир време када је коментар остављен и број гласова за потребе сортирања.

Корисник затим може да промени смер сортирања на или „Oldest“ или „Newest First“ у корисничком интерфејсу виџета за коментаре.

Међутим, подразумевано понашање можемо променити на било коју од ове три опције. На пример, ако желите да прикажете најстарије коментаре прво:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Постављамо вредност **defaultSortDirection** на "OF" да бисмо подесили смер на "OF".

За смер сортирања „најновији први“ урадили бисмо следеће:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Допуштене вредности за **defaultSortDirection** су:

- MR: "Најновије"
- NF: "Најновији први"
- OF: "Најстарији први"

Ово се може урадити и без кода. На страници за прилагођавање виџета, погледајте одељак „Подразумевани смер сортирања“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Имајте на уму да се коментари на свакој страници за сваки смер сортирања претходно израчунавају, тако да сви смерови сортирања имају исте перформансе.

---