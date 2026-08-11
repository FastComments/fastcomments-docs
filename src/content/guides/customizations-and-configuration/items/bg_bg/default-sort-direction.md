[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

По подразбиране FastComments ще сортира коментарите по посоката за сортиране "Най-относими".

Сортирането "Най-относими" взема предвид времето, в което е оставен коментарът, и броя гласове при сортирането.

Потребителят може след това да промени посоката за сортиране на Oldest или Newest First в потребителския интерфейс на уиджета за коментари.

Въпреки това можем да променим подразбиращата се стойност на която и да е от трите. Например, ако искате да покажете най-старите коментари първо:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Промяна на подразбиращото се сортиране към най-старите първо'; code-example-end]

Задаваме стойността на **defaultSortDirection** на "OF", за да зададем посоката на "OF".

За посоката за сортиране "Newest First", ще направим следното:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Промяна на подразбиращото се сортиране към най-новите първо'; code-example-end]

Валидните стойности за **defaultSortDirection** са:

- MR: "Най-скорошен"
- NF: "Най-нови първо"
- OF: "Най-стари първо"

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте секцията "Default Sort Direction" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Селектор за подразбираща се посока за сортиране, предлагащ Най-относими, Newest First, и Oldest First'; title='Промяна на подразбиращата се посока за сортиране' app-screenshot-end]

Обърнете внимание, че коментарите на всяка страница за всяка посока за сортиране са предварително изчислени, така че всички посоки за сортиране имат еднаква производителност.