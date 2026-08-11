[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentowanie może być zablokowane, aby nie można było zostawiać nowych komentarzy ani głosów, poprzez ustawienie flagi readonly na true.

Komentarze nie będą również mogły być edytowane ani usuwane.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widgetu, dla całej domeny lub strony:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Ustawienie zapobiegające nowym odpowiedziom na stronie dostosowywania widgetu, które blokuje wątek dla domeny lub strony'; title='Ustawianie wątku komentarzy w tryb tylko do odczytu' app-screenshot-end]

## Update!

Od listopada 2022 r. wątki mogą być blokowane lub odblokowywane **na żywo** przez administratorów i moderatorów za pomocą menu z trzema kropkami nad obszarem odpowiedzi.

Spowoduje to zapobieganie nowym komentarzom, jednocześnie umożliwiając głosowanie oraz pozwalając użytkownikom usuwać ich komentarze, jeśli zechcą, podczas gdy `readonly` nie pozwala na te czynności. 

Odpowiada to polu `isClosed` w API `Page`.

---