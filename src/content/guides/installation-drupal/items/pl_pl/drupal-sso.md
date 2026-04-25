FastComments integruje się z systemem użytkowników Drupala za pomocą SSO (single sign-on). Twoi użytkownicy logują się na swojej stronie Drupal, a moduł automatycznie przekazuje ich tożsamość do FastComments. Brak dodatkowych kont do tworzenia, brak konieczności przeprowadzania początkowej synchronizacji.

The module supports three SSO modes, set under `Administration > Configuration > Content > FastComments`.

### Brak

Brak SSO. Użytkownicy komentują jako goście lub tworzą konto FastComments. Użyj tego, jeśli Twoja strona jest publiczna i nie musisz powiązywać komentarzy z użytkownikami Drupala.

### Prosty

Przekazuje do FastComments nazwę użytkownika Drupala, adres e-mail i awatar bez weryfikacji po stronie serwera. Nie jest potrzebny API Secret. Dobre dla serwisów wewnętrznych lub o niskim ryzyku.

### Bezpieczny (zalecany)

Używa [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) do weryfikacji tożsamości każdego użytkownika z FastComments. To jest tryb, którego chcesz użyć, gdy masz skonfigurowany API Secret, i jest to jedyny tryb, który zapobiega podszywaniu się odwiedzającego pod innego użytkownika.

Tożsamość użytkownika jest przekazywana do FastComments za każdym razem, gdy użytkownik przegląda wątek komentarzy. Nie ma potrzeby uruchamiania początkowej ani ciągłej synchronizacji.

<sup>(Opcjonalne)</sup> Dodaj swoich administratorów do [Użytkownicy i administratorzy](https://fastcomments.com/auth/my-account/users) i moderatorów do [Moderatorzy komentarzy](https://fastcomments.com/auth/my-account/moderate-comments/moderators), aby poprawić ich doświadczenie i włączyć śledzenie statystyk dla moderatorów.

Aby lepiej zrozumieć, jak działa SSO, zobacz [sekcję SSO](/guide-customizations-and-configuration.html#sso) w dokumentacji dotyczącej dostosowań.

---