[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments pokaże ikonę dzwonka powiadomień w prawym górnym rogu obszaru komentarzy.

Ten dzwonek zmieni kolor na czerwony i pokaże liczbę powiadomień użytkownika. Przykładowe powiadomienia to:

- Użytkownik odpowiedział Ci.
- Użytkownik odpowiedział w wątku, w którym skomentowałeś(-aś).
- Użytkownik zagłosował pozytywnie na Twój komentarz.
- Użytkownik odpowiedział na stronie, którą subskrybujesz.

Dzwonek powiadomień umożliwia również subskrybowanie całej strony.

Można jednak całkowicie wyłączyć dzwonek powiadomień:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

To można również zrobić bez użycia kodu. Na stronie dostosowywania widżetu zobacz sekcję "Wyłącz dzwonek powiadomień".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---