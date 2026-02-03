FastComments pozwala wymagać od komentujących po raz pierwszy zaakceptowania Warunków świadczenia usług przed przesłaniem komentarza.

When enabled:
- **Użytkownicy anonimowi** będą widzieć pole wyboru akceptacji Warunków świadczenia usług przy każdym komentowaniu
- **Uwierzytelnieni użytkownicy** zobaczą pole wyboru tylko przy swoim pierwszym komentarzu, lub gdy zaktualizujesz swoje Warunki świadczenia usług

### Configuration

Przejdź do strony dostosowywania widżetu i włącz pole wyboru "Require Terms of Service acceptance". Po włączeniu zobaczysz następujące opcje:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: Domyślnie pole wyboru wyświetla "I agree to the Terms of Service and Privacy Policy" z linkami do obu dokumentów. Wybierz "Customize text per locale", aby dostarczyć własny tekst dla każdego języka.
- **TOS Last Updated Date**: Gdy zaktualizujesz swoje Warunki świadczenia usług, ustaw tę datę. Użytkownicy, którzy zaakceptowali je przed tą datą, będą musieli ponownie zaakceptować.

### How It Works

- Znacznik czasu akceptacji Warunków świadczenia usług jest przechowywany dla każdego użytkownika i każdego komentarza
- Gdy użytkownik zaakceptuje Warunki, data jest zapisywana w jego profilu użytkownika (per-tenant)
- Jeśli ustawisz datę "Last Updated", która jest późniejsza niż data akceptacji użytkownika, użytkownicy będą musieli ponownie zaakceptować
- Dla użytkowników anonimowych, których nie można śledzić, pole wyboru pojawia się przy każdym przesłaniu komentarza