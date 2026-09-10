## Połącz swoje konto

1. W Zapierze dodaj krok FastComments do Zapa, lub otwórz stronę aplikacji FastComments w katalogu aplikacji Zapier.  
2. Wybierz **Sign in to FastComments**. Zapier najpierw pyta o region: wybierz **United States**, chyba że Twoje konto zostało utworzone w regionie UE (`eu.fastcomments.com`).  
3. Otwiera się okno FastComments. Zaloguj się, jeśli nie jesteś już zalogowany.  
4. Przejrzyj stronę zgody. Pokazuje ona aplikację Zapier, konto, z którym zostanie połączona, oraz żądane uprawnienia (odczyt i zapis). Wybierz **Approve**.  
5. Zapier zapisuje połączenie i oznacza je nazwą Twojej witryny oraz nazwą użytkownika.  

Połączenie używa OAuth. Żaden klucz API nie jest kopiowany do Zapiera, a token przechowywany przez Zapier działa wyłącznie dla konta, które zatwierdziłeś.

## Kto może połączyć

Osoba zatwierdzająca połączenie musi być **API admin** na koncie FastComments. Właściciele kont mają to uprawnienie; innym członkom zespołu można je przyznać na stronie Users. Osoba bez tego uprawnienia zobaczy stronę „you do not have permission” zamiast formularza zgody.

## Łączenie właściwej witryny

Strona zgody łączy konto, do którego jesteś aktualnie zalogowany. Jeśli zarządzasz kilkoma kontami, przełącz się na właściwe za pomocą przełącznika kont przed zatwierdzeniem, lub użyj linku **switch account** na stronie zgody. Etykieta połączenia w Zapierze wyświetla nazwę witryny, więc błędny wybór łatwo zauważyć.

## Przeglądanie i odwoływanie dostępu

Każde połączenie pojawia się w sekcji **Connected Apps** w panelu FastComments, wraz z uprawnieniami, które posiada, oraz informacją, kiedy było ostatnio używane. Odwołanie go tam natychmiast rozłącza Zapiera; każdy Zap używający tego połączenia przestaje działać, dopóki nie zostanie ponownie połączony. Możesz także usunąć połączenie po stronie Zapiera w sekcji **My Apps**.