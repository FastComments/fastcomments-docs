---
Narzędzie Edit pozwala agentowi zastąpić tekst istniejącego komentarza. Jest ono destrukcyjne w sposób, w jaki większość innych narzędzi moderacyjnych nie jest: nadpisuje treść stworzoną przez użytkownika. Stosuj je tylko w wąskich, jednoznacznych przypadkach.

### Co robi

Agent przekazuje ID komentarza i treść zastępczą. Platforma zapisuje nowy tekst w komentarzu i rejestruje wpis `TextChanged` w dzienniku audytu komentarza, zachowując zarówno stary, jak i nowy tekst. Oryginał nigdy nie zostaje utracony - moderatorzy mogą przeczytać, co komentarz zawierał przed edycją przez agenta.

Zastąpienie przechodzi przez tę samą ścieżkę renderowania co edycja dokonana przez człowieka: maskowanie przekleństw, parsowanie wzmianek, ekstrakcja hashtagów oraz obsługa linków/obrazów zachowują się dokładnie tak, jakby nowy tekst został przesłany przez pierwotnego autora.

### Zakres

Podobnie jak każde narzędzie modyfikujące komentarze, Edit jest ograniczone do allowlisty wyzwalacza - agent może edytować tylko komentarz, na którym wyzwalacz został uruchomiony, jego rodzica lub inny komentarz objęty zakresem z tego samego kontekstu wyzwalacza. Próba wstrzyknięcia polecenia typu "edit comment XYZ", gdzie XYZ jest niezwiązane, zostanie odrzucona po stronie serwera zanim uruchomi się wykonawca.

### Pętle

Kiedy agent edytuje komentarz, platforma wyzwala trigger `COMMENT_EDIT` tak jak przy edycji przez człowieka, ale **wstrzymuje dystrybucję do innych agentów**. Zapobiega to sytuacji, w której dwaj agenci nasłuchujący `COMMENT_EDIT` odbijają się nawzajem, reagując na edycje jednego i drugiego.

### Kiedy zezwolić

Dla agentów zajmujących się redakcją PII (danych osobowych), lub dla agentów samodzielnie edytujących podsumowania/digesty. Większość agentów moderacyjnych **nie** potrzebuje tego narzędzia - mark-spam, warn, and ban obejmują typowy cykl życia.

### Zatwierdzenia

**Zdecydowanie rozważ ograniczenie dostępu poprzez zatwierdzenie**, szczególnie podczas budowania zaufania do agenta. Agent przepisujący słowa użytkownika to działanie, które społeczność zauważy i na które zareaguje, i jest trudniejsze do "odwrócenia" reputacyjnie niż usunięcie.

### Zobacz także

- [Trigger: Comment Edited](#trigger-comment-edit) - wyzwalacz uruchamiany, gdy tekst komentarza się zmienia.
- [Approval Workflow](#approval-workflow) - jak objąć narzędzie przeglądem wykonywanym przez człowieka.

---