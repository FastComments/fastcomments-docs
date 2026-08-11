[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments wyświetla dzwonek powiadomień w prawym górnym rogu obszaru komentarzy.

Ten dzwonek zmieni kolor na czerwony i pokaże liczbę powiadomień, które ma użytkownik. Przykładowe powiadomienia to:

- Użytkownik odpowiedział Tobie.
- Użytkownik odpowiedział w wątku, w którym komentowałeś.
- Użytkownik przyznał Twój komentarz.
- Użytkownik odpowiedział na stronę, którą subskrybujesz.

Dzwonek powiadomień zapewnia także mechanizm subskrypcji całej strony.

Jednak możemy całkowicie wyłączyć dzwonek powiadomień:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Można to zrobić również bez kodu. Na stronie dostosowywania widgetu zobacz sekcję „Disable Notification Bell”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Strona dostosowywania widgetu z zaznaczonym polem wyboru Wyłącz dzwonek powiadomień'; title='Wyłącz dzwonek powiadomień' app-screenshot-end]