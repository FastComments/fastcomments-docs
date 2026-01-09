[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentowanie można zablokować, tak aby nie można było zostawiać nowych komentarzy ani głosów, ustawiając flagę readonly na true.

Komentarze również nie będą mogły być edytowane ani usuwane.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widżetu, dla całej domeny lub pojedynczej strony:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Aktualizacja!

Od listopada 2022 wątki mogą być blokowane lub odblokowywane **na żywo** przez administratorów i moderatorów za pomocą menu z trzema kropkami nad polem odpowiedzi.

To uniemożliwi dodawanie nowych komentarzy, jednocześnie umożliwiając głosowanie oraz pozwalając użytkownikom usunąć swoje komentarze, jeśli chcą, natomiast `readonly` tego nie pozwala. 

To odpowiada polu `isClosed` w API `Page`.