[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Som standard vil FastComments sortere kommentarer efter sorteringsretningen "Most Relevant".

Most Relevant-sortering tager tidspunktet for hvornår kommentaren blev skrevet og antallet af stemmer i betragtning ved sortering.

Brugeren kan derefter ændre sorteringsretningen til enten Oldest eller Newest First i kommentar-widgetens UI.

Vi kan dog ændre standarden til en af de tre. For eksempel, hvis du vil vise de ældste kommentarer først:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Ændring af standard sortering til ældste først'; code-example-end]

Vi sætter værdien af **defaultSortDirection** til "OF" for at indstille retningen til "OF".

For sorteringsretningen newest-first ville vi gøre følgende:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Ændring af standard sortering til nyeste først'; code-example-end]

De gyldige værdier for **defaultSortDirection** er:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Dette kan også gøres uden kode. På widget-tilpasningssiden, se sektionen "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Standard sorteringsretning vælger, der tilbyder Most Relevant, Newest First og Oldest First'; title='Ændring af standard sorteringsretning' app-screenshot-end]

Bemærk, at kommentarerne på hver side for hver sorteringsretning er forudberegnet, så alle sorteringsretninger har samme ydeevne.