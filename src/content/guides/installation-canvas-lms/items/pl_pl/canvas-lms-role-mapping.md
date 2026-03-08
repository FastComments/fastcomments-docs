Role w Canvas są automatycznie mapowane na role FastComments podczas uruchomienia LTI. Nie jest wymagana ręczna konfiguracja.

#### Mapowanie ról

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | Pełny dostęp do konta, zarządzanie wszystkimi komentarzami i ustawieniami |
| **Instructor** | Moderator | Edytowanie i usuwanie komentarzy, przypinanie wątków, zarządzanie dyskusjami |
| **Learner** | Commenter | Publikowanie komentarzy, odpowiadanie, głosowanie oraz używanie wzmianek |

#### Jak to działa

Kiedy użytkownik uruchamia FastComments z Canvas, protokół LTI 1.3 zawiera jego rolę w Canvas. FastComments odczytuje tę rolę i automatycznie przypisuje odpowiednie uprawnienia.

Jeśli użytkownik ma wiele ról (np. użytkownik, który pełni rolę Instructor i jednocześnie rolę Admin), używana jest rola o najwyższych uprawnieniach.

---