[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Liczba komentarzy wyświetlana u góry widżetu komentarzy może albo pokazywać wszystkie "top-level" komentarze, czyli te odpowiedzi, które są odpowiedziami bezpośrednio do strony lub artykułu, albo może być liczbą **wszystkich** zagnieżdżonych komentarzy.

Domyślnie jest to `true` - jest to liczba tej drugiej kategorii - wszystkich komentarzy. W starszych wersjach widżetu komentarzy wartością domyślną jest `false`.

Możemy zmienić zachowanie, tak aby była to liczba **wszystkich** zagnieżdżonych komentarzy, ustawiając flagę **countAll** na true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Jeśli chcielibyśmy, aby liczba odzwierciedlała tylko komentarze najwyższego poziomu, ustawiamy flagę na false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Obecnie nie można tego dostosować bez zmiany kodu.

---