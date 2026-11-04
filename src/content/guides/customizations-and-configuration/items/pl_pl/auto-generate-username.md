Gdy użytkownicy komentują lub głosują, a nie są zalogowani, zostaną poproszeni o podanie swojego adresu e-mail i nazwy użytkownika.

Dla niektórych stron proszenie odwiedzającego o wymyślenie unikalnej nazwy użytkownika jest barierą, szczególnie na urządzeniach mobilnych. FastComments może
wygenerować neutralną nazwę użytkownika dla każdego nowego odwiedzającego i wstępnie ją wypełnić w polu nazwy użytkownika, np. `BraveOtter4172`.

Odwiedzający może zostawić ją bez zmian lub zastąpić ją nazwą według własnego wyboru.

Można to włączyć z interfejsu Customization, w ustawieniu o nazwie `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Opcja Generate Usernames Automatically w interfejsie dostosowywania widżetu'; title='Generate Usernames Automatically' app-screenshot-end]

#### Jak to działa

- Każda wygenerowana nazwa jest unikalna. Jest sprawdzana względem istniejących kont i zarezerwowana dla sesji przeglądarki danego odwiedzającego, więc dwóm odwiedzającym nie zostanie zaproponowana ta sama nazwa.
- Nazwa jest generowana tylko dla odwiedzających, którzy jeszcze jej nie mają. Zalogowani użytkownicy, użytkownicy SSO oraz odwiedzający, którzy już komentowali, zachowują swoją istniejącą nazwę.
- Działa z komentowaniem anonimowym lub bez niego ([anonymous commenting](/guide-customizations-and-configuration.html#allow-anon)). Przy wyłączonym komentowaniu anonimowym odwiedzający nadal podaje swój adres e-mail, ale nie musi już wymyślać nazwy użytkownika.
- Powracający odwiedzający, który poda adres e-mail używany wcześniej, zostanie dopasowany do swojego istniejącego konta i zachowa nazwę z tego konta.
- Jeśli ustawiona jest również opcja [Default Username](/guide-customizations-and-configuration.html#default-username), wygenerowana nazwa ma pierwszeństwo.