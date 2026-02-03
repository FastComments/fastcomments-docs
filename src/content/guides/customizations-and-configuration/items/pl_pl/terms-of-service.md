FastComments umożliwia wymaganie od osób komentujących po raz pierwszy zaakceptowania Twoich Warunków korzystania z usługi (Terms of Service) przed wysłaniem komentarza.

When enabled:
- **Użytkownicy anonimowi** będą widzieć pole wyboru dotyczące Warunków korzystania z usługi za każdym razem, gdy skomentują
- **Uwierzytelnieni użytkownicy** zobaczą pole wyboru tylko przy pierwszym komentarzu lub gdy zaktualizujesz Warunki korzystania z usługi

### Włączanie Warunków korzystania z usługi

Przejdź do strony dostosowywania widżetu i włącz pole wyboru "Wymagaj akceptacji Warunków korzystania z usługi":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Dostosowywanie tekstu Warunków korzystania z usługi

Domyślnie pole wyboru wyświetla "I agree to the Terms of Service and Privacy Policy" z linkami do obu dokumentów. Możesz dostosować ten tekst dla poszczególnych lokalizacji, jeśli to konieczne:

1. Wybierz "Customize text per locale"
2. Wybierz lokalizację z rozwijanego menu i wprowadź własny tekst

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Aktualizowanie Warunków korzystania z usługi

Gdy zaktualizujesz Warunki korzystania z usługi, ustaw datę "Last Updated". Użytkownicy, którzy zaakceptowali Warunki przed tą datą, będą musieli zaakceptować je ponownie:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Jak to działa

- Znacznik czasu akceptacji Warunków jest przechowywany dla każdego użytkownika i każdego komentarza
- Gdy użytkownik akceptuje Warunki, data jest zapisywana w jego profilu użytkownika (dla danego tenanta)
- Jeśli ustawisz datę "Last Updated" późniejszą niż data akceptacji użytkownika, będą musieli zaakceptować ponownie
- Dla użytkowników anonimowych, których nie można śledzić, pole wyboru pojawia się przy każdym wysłaniu komentarza

---