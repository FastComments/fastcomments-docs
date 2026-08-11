FastComments umożliwia wymóg akceptacji Warunków świadczenia usług przez komentujących po raz pierwszy przed wysłaniem komentarza.

Po włączeniu:
- **Użytkownicy anonimowi** zobaczą pole wyboru TOS przy każdym komentarzu
- **Użytkownicy uwierzytelnieni** zobaczą pole wyboru tylko przy swoim pierwszym komentarzu lub gdy zaktualizujesz swoje TOS

### Konfiguracja

Przejdź do strony dostosowywania widżetu i włącz pole wyboru „Wymagaj akceptacji Warunków świadczenia usług”. Po włączeniu zobaczysz następujące opcje:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Panel Warunków świadczenia usług pokazujący selektor trybu tekstu TOS i pole daty ostatniej aktualizacji'; title='Opcje Warunków świadczenia usług' app-screenshot-end]

- **Tryb tekstu TOS**: Domyślnie pole wyboru wyświetla „Zgadzam się z Warunkami świadczenia usług i Polityką prywatności” z odnośnikami do obu dokumentów. Wybierz „Dostosuj tekst dla każdego języka”, aby podać własny tekst dla każdego języka.
- **Data ostatniej aktualizacji TOS**: Gdy aktualizujesz swoje Warunki świadczenia usług, ustaw tę datę. Użytkownicy, którzy zaakceptowali je przed tą datą, będą musieli zaakceptować ponownie.

### Jak to działa

- Znacznik czasu akceptacji TOS jest przechowywany dla każdego użytkownika i każdego komentarza
- Gdy użytkownik akceptuje TOS, data jest zapisywana w jego profilu użytkownika (per-tenant)
- Jeśli ustawisz datę „Ostatniej aktualizacji”, która jest późniejsza niż data akceptacji użytkownika, będzie on musiał ponownie zaakceptować
- Dla anonimowych użytkowników, których nie można śledzić, pole wyboru pojawia się przy każdym wysyłaniu komentarza

---