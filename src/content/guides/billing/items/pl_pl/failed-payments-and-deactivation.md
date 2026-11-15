Płatność może się nie powieść, ponieważ karta wygasła, została wymieniona, nie ma środków, albo bank zablokował obciążenie. FastComments daje Ci czas na naprawienie sytuacji, zanim cokolwiek przestanie działać, i powiadamia osoby, które mogą to naprawić.

### Co się dzieje

1. **Obciążenie się nie powodzi.** Faktura pozostaje w stanie `Unpaid` w [Billing History](https://fastcomments.com/auth/my-account/billing-history) z przyciskiem `Pay Now`. W przypadku planu stałego Stripe może ponowić próbę obciążenia. W przypadku Flex, FastComments ponawia każdą niezapłaconą fakturę starszą niż okres karencji co noc.
2. **Wysyłane jest ostrzeżenie.** Każdej nocy FastComments sprawdza każde konto. Jeśli faktura Flex pozostaje niezapłacona po okresie karencji i ponowna próba się nie powiodła, albo subskrypcja w planie stałym zakończyła się, ponieważ nie udało się jej pobrać, każdy użytkownik z uprawnieniem `Billing Admin` otrzymuje e-mail `Your FastComments Tenant Will Be Deactivated Soon!` z datą, a konto jest planowane do dezaktywacji po upływie jednego okresu karencji. W międzyczasie widżet nadal działa.
3. **Dezaktywacja.** Jeśli do zaplanowanej daty nic się nie zmieni, konto zostaje dezaktywowane: komentarze są ukrywane na Twojej stronie, a odwiedzający widzą komunikat `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Administratorzy rozliczeń otrzymują ponownie e-mail. Twoje dane są zachowywane.

Okres karencji wynosi jeden tydzień dla większości kont i dwa tygodnie dla kont starszych niż rok. W przypadku Flex stosuje się go dwukrotnie — raz przed ostrzeżeniem i raz między ostrzeżeniem a dezaktywacją — więc od nieudanego obciążenia do ukrycia komentarzy mija co najmniej dwa tygodnie.

### Jak to naprawić

Zapisz działającą kartę na stronie [Billing Info](https://fastcomments.com/auth/my-account/billing-info). To samo w sobie natychmiast reaktywuje konto i anuluje zaplanowaną dezaktywację; niezapłacona faktura jest następnie pobierana nową kartą — natychmiast w planie stałym i podczas nocnego sprawdzania w przypadku Flex. Możesz też zapłacić fakturę od razu przyciskiem `Pay Now` w Billing History.

Jeśli karta zapisana na koncie jest prawidłowa, a niepowodzenie było tymczasowe, nie musisz nic robić: nocne sprawdzanie zauważy opłaconą fakturę lub aktywną subskrypcję, anuluje dezaktywację i wyśle e-mail do administratorów rozliczeń z potwierdzeniem. Konto już zdeaktywowane jest reaktywowane w ten sam sposób.

### Kilka faktur jednocześnie

Jeśli Twoja karta była nieprawidłowa przez kilka miesięcy, a potem została naprawiona, każdy niezapłacony miesiąc jest pobierany osobno, więc możesz zobaczyć kilka obciążeń tego samego dnia. Sprawdź okres rozliczeniowy na każdej fakturze w Billing History: dotyczą one różnych miesięcy, a nie są duplikatami.

### Nadal widzisz ostrzeżenie o rozliczeniach po zapłacie?

Ostrzeżenie dotyczy konkretnego konta. Jeśli zapłaciłeś, ale strona nadal wyświetla komunikat `billing information is out of date`, widżet na tej stronie najprawdopodobniej używa innego konta niż to, na którym dokonałeś płatności. Dzieje się tak, gdy strona została skonfigurowana dwukrotnie, albo gdy istnieje jedno konto na `fastcomments.com` i inne na wdrożeniu UE pod adresem `eu.fastcomments.com`, które są rozliczane osobno. Porównaj `tenantId` w kodzie na swojej stronie z tym w `Settings -> API/SSO`, albo [skontaktuj się z pomocą techniczną](https://fastcomments.com/auth/my-account/help), a wyjaśnimy sprawę.