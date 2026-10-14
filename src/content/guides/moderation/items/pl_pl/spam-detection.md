Domyślnie FastComments zawiera trenowalny system wykrywania spamu.

W miarę moderowania komentarzy i oznaczania ich jako **Spam**, lub oznaczania automatycznie wykrytych jako **Spam** jako **Not Spam**, system wykrywania spamu będzie uczył się na podstawie tych działań, aby dokładniej określać, co chcesz traktować jako spam.

Komentarze oznaczone jako **Spam** nie będą automatycznie zatwierdzane, więc nie będą widoczne, dopóki nie zostaną wyraźnie oznaczone jako **Not Spam**.

Wykrywanie spamu można wyłączyć na stronie Ustawień moderacji komentarzy.

### Różne detektory spamu

FastComments obsługuje trzy sposoby wykrywania spamu:

1. Tradycyjny klasyfikator Naïve-Bayes, który jest ciągle trenowany i który jest współdzielony między wszystkimi tenantami FastComments.com.
2. Tradycyjny klasyfikator Naïve-Bayes, który jest ciągle trenowany i który jest **isolated** do Twojego tenanta.
3. Korzystanie z LLM (dużego modelu językowego).

Wszyscy mają dostęp do współdzielonych i izolowanych klasyfikatorów Naïve-Bayes.

Opcja LLM jest dostępna do wyboru na stronie Ustawień moderacji komentarzy, jeśli korzystasz z rozliczeń Flex, ponieważ rozliczana jest na podstawie użytych tokenów.

### Wskaźnik zaufania

FastComments dostosowuje filtr spamu dla użytkownika na podstawie poziomu jego zaufania dla danej witryny.

Na przykład, jeśli administratorzy przypięli wiele ich komentarzy, prawdopodobnie jest to bardzo zaufany użytkownik. Albo jeśli jest członkiem serwisu od długiego czasu i ma wiele komentarzy, jego wskaźnik zaufania również może być wysoki.

### SSO

Komentarze publikowane przez użytkowników SSO mogą być uznawane za spam i będą sprawdzane jako takie. Wyjątkiem jest sytuacja, gdy użytkownik SSO ma ten sam adres e-mail co użytkownik tenanta, który posiada jedną lub więcej z następujących uprawnień:

- Account Owner
- Super Admin
- Comment Moderator Admin

Użytkownicy SSO z tymi uprawnieniami nie będą mieli swoich komentarzy sprawdzanych pod kątem spamu.

### Powtarzające się wiadomości

FastComments wykryje i zapobiegnie powtarzającym się wiadomościom. Wykryje również powtarzające się wiadomości bardzo do siebie podobne, aby zapobiegać spamowi. Tego nie da się wyłączyć, ponieważ zapobiega to wykorzystywaniu naszej platformy do nadużyć. Jeśli masz wysoki wskaźnik zaufania, jest to brane pod uwagę przy zapobieganiu powtarzającym się wiadomościom.