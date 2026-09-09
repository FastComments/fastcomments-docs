[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Domyślnie, jeśli użytkownik wpisze komentarz, a następnie odświeży stronę, zamknie kartę lub opuści ją przed wysłaniem, szkic zostaje utracony w ciszy.

Ustawienie **warnOnUnsavedComment** na true powoduje, że przeglądarka pyta użytkownika o potwierdzenie przed opuszczeniem strony, gdy jakiekolwiek pole komentarza lub trwająca edycja nadal zawiera tekst. Po wysłaniu komentarza tekst jest usuwany, więc żadne ostrzeżenie nie jest wyświetlane.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Ostrzeżenie o niezapisanym komentarzu'; code-example-end]

Powiadomienie korzysta z własnego okna dialogowego przeglądarki. Nowoczesne przeglądarki wyświetlają własny tekst i ignorują niestandardowe treści, więc wiadomość nie może być dostosowana.

Ta opcja ładuje małe rozszerzenie na żądanie, więc nie dodaje nic do widgetu na stronach, które jej nie włączają.