[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Som standard sorterer FastComments kommentarer efter "Mest relevante" sorteringsretning.

Sorteringen "Mest relevante" tager tidspunktet, hvor kommentaren blev skrevet, og antallet af stemmer i betragtning ved sortering.

Brugeren kan derefter ændre sorteringsretningen til enten "Ældste først" eller "Nyeste først" i kommentarswidgetens brugerflade.

Vi kan dog ændre standarden til en af de tre. For eksempel, hvis du ønsker at vise de ældste kommentarer først:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Vi sætter værdien af **defaultSortDirection** til "OF" for at sætte retningen til "OF".

For sorteringsretningen "Nyeste først" ville vi gøre følgende:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

De gyldige værdier for **defaultSortDirection** er:

- MR: "Mest nylig"
- NF: "Nyeste først"
- OF: "Ældste først"

Dette kan også gøres uden kode. På widgetens tilpasningsside, se afsnittet "Standard sorteringsretning".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Bemærk, at kommentarerne på hver side for hver sorteringsretning er forudberegnede, så alle sorteringsretninger har samme ydeevne.