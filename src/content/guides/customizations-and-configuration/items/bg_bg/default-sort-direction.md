[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

По подразбиране, FastComments сортира коментари по посоката на сортиране "Most Relevant".

Сортирането по Most Relevant взема предвид времето, когато е оставен коментарът, и броя гласове при сортиране.

Потребителят след това може да промени посоката на сортиране на или Oldest, или Newest First в потребителския интерфейс на уиджета за коментари.

Въпреки това, можем да променим подразбиращото се на всяка от трите. Например ако искате да показвате първо най-старите коментари:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Задаваме стойността на **defaultSortDirection** на "OF", за да зададем посоката на "OF".

За посоката на сортиране newest-first, бихме направили следното:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Валидните стойности за **defaultSortDirection** са:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Това може да се направи и без код. В страницата за персонализиране на уиджета вижте секцията "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Имайте предвид, че коментарите на всяка страница за всяка посока на сортиране са предварително пресметнати, така че всички посоки на сортиране имат една и съща производителност.