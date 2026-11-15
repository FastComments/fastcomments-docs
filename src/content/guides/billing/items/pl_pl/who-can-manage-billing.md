Rozliczenia są ograniczone do dwóch rodzajów użytkowników:

- `Account Owner`, czyli użytkownik, który utworzył konto.
- Każdy użytkownik z uprawnieniem `Billing Admin`.

To oni są jedynymi użytkownikami, którzy mogą zmienić plan, zapisać lub zastąpić kartę, otworzyć `Invoice Settings`, ustawić limity wydatków Flex oraz dodać kredyt. Każdy inny, kto spróbuje, zobaczy komunikat `Only the account owner or a billing admin can change the plan or payment method.`

E-maile rozliczeniowe, takie jak przypomnienie w połowie okresu próbnego, ostrzeżenia o nieudanej płatności i powiadomienia o dezaktywacji, są wysyłane do użytkowników z uprawnieniem `Billing Admin`. Przypisz to uprawnienie osobie, która powinna je otrzymywać, na przykład komuś z działu finansów, a będzie dostawać e-maile bez potrzeby jakiegokolwiek innego dostępu do panelu. Przypomnienia wysyłane po zakończeniu okresu próbnego bez danych rozliczeniowych trafiają do każdego użytkownika panelu na koncie. Każdy z tych e-maili zawiera link do rezygnacji z subskrypcji; użytkownik, który zrezygnuje z powiadomień konta, przestanie je otrzymywać.

Faktury same w sobie trafiają na adres `Billing Email` w [Invoice Settings](#invoice-settings), który w ogóle nie musi należeć do użytkownika panelu.

Strona [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) wymaga natomiast uprawnienia `Analytics Admin`, ponieważ pokazuje użycie, a nie szczegóły płatności.

Aby nadać lub odebrać uprawnienie, otwórz stronę `Users` i edytuj użytkownika. Zobacz [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). W przypadku Flex pamiętaj, że każdy dodatkowy administrator to rozliczane miejsce, zobacz [Admin and Moderator Seats](#admin-and-moderator-seats).

Jeśli jesteś zalogowany i nie widzisz w ogóle sekcji `Billing Info`, albo powoduje ona przekierowanie na stronę główną, to znaczy, że jesteś zalogowany jako konto komentującego, a nie użytkownik panelu. Wyloguj się i zaloguj na konto, do którego należy witryna.