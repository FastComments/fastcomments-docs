FastComments współpracuje ze stronami dostępnymi tylko dla członków, używając mechanizmu zwanego SSO, czyli single-sign-on. Twoi członkowie logują się na twojej stronie WordPress, ale
nie muszą martwić się o tworzenie konta w FastComments ani logowanie przez media społecznościowe, żeby komentować. Jeśli twoi członkowie (w tym administratorzy) są zalogowani na
twojej stronie WordPress, będą mogli komentować. Twoi administratorzy i moderatorzy będą mogli moderować komentarze bezpośrednio z wpisów na Twoim blogu WordPress, jak również.

<sup>(Optional)</sup> Remember to also add your administrators to [Użytkownicy i administratorzy](https://fastcomments.com/auth/my-account/users) and moderators to [Moderatorzy komentarzy](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
to improve their experience and enable stat tracking for moderators.

SSO można włączyć, przechodząc do panelu wtyczki i klikając „Ustawienia SSO”.

Najpierw musisz włączyć funkcję „Każdy może się zarejestrować” na swojej stronie.

Wszystkie informacje o użytkownikach są bezproblemowo i bezpiecznie przesyłane do FastComments za każdym razem, gdy użytkownik wyświetla wątek komentarzy za pomocą [HMAC](https://en.wikipedia.org/wiki/HMAC).

Nie ma potrzeby uruchamiania początkowej ani ciągłej synchronizacji w celu skopiowania członków na serwery FastComments. Odbywa się to automatycznie, gdy oglądają wątki komentarzy, otwierając wpis na blogu.

---

## Nazwy i awatary

Wtyczka automatycznie zaktualizuje nazwę wyświetlaną użytkownika i awatar we wszystkich jego komentarzach w FastComments, gdy wyświetli którykolwiek wątek komentarzy. Awatary są obsługiwane przez Gravatar lub dowolną wtyczkę do zarządzania awatarami w WordPress, ponieważ wtyczka wywoła `get_avatar_url`.