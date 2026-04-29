Alerty e-mail o budżecie są wysyłane, gdy wydatki agenta przekroczą konfigurowalny procent jego limitu. Trafiają do osób, które opłacają rachunek.

### Jak działają alerty

Każdy agent ma pole **Alert thresholds** w formularzu edycji. Domyślnie to `80%` i `100%`. Możesz zaznaczać lub odznaczać poszczególne progi oraz dodawać inne procenty.

Gdy wydatki agenta w danym zakresie (dziennym lub miesięcznym) po raz pierwszy w tym okresie przekroczą próg, platforma wysyła jeden e-mail na odbiorcę. Ponowne przekroczenie tego samego progu później w tym samym okresie (np. wydatki spadły poniżej 80% i ponownie wzrosły) **nie** powoduje ponownego wysłania.

To dotyczy pojedynczego okresu: nowy dzienny reset zeruje logikę wykrywania przekroczeń na ten dzień.

### Alerty w zakresie konta (tenant)

Tenant (konto) ma własne limity dzienne i miesięczne. Alerty na poziomie konta pojawiają się przy stałych progach (`80%` i `100%`). Nie są one konfigurowalne per agent, ponieważ dotyczą całego konta.

### Odbiorcy

Alerty o budżecie są wysyłane do:

- Każdego użytkownika oznaczonego jako **Super administrator** na koncie.
- Każdego użytkownika oznaczonego jako **Administrator rozliczeń** na koncie.

To obejmuje sumę obu ról — użytkownik posiadający obie role otrzyma jeden e-mail.

### Dlaczego obie role

Superadministratorzy to zazwyczaj operatorzy, którzy muszą wiedzieć, że agent zbliża się do limitu. Administratorzy rozliczeń są właścicielami faktury i muszą być informowani o skokach kosztów niezależnie od tego, czy zarządzają agentami na co dzień. Aby faktycznie edytować agenta (zwiększyć limit, wstrzymać go), odbiorca potrzebuje również roli **Administrator dostosowań** — która kontroluje dostęp do strony edycji agenta.

### Rezygnacja pojedynczego użytkownika

Odbiorcy, którzy zrezygnowali z powiadomień administracyjnych w swoim profilu, są pomijani. To ten sam przełącznik rezygnacji, który kontroluje inne powiadomienia administratora.

Jeżeli **wszyscy** odbiorcy zrezygnowali, alert jest logowany (poziom ostrzeżenia) i e-mail nie jest wysyłany.

### Zawartość e-maila

E-mail zawiera:

- **nazwę wyświetlaną agenta** oraz nazwę wewnętrzną.
- **zakres**, który został przekroczony (np. "dzienny budżet agenta", "miesięczny budżet agenta", "dzienny budżet konta", "miesięczny budżet konta").
- **przekroczony procent progu**.
- **wykorzystanie** w walucie konta.
- **limit** w walucie konta.
- **jednoklikowy podpisany link logowania**, który zabiera odbiorcę bezpośrednio do:
  - strony edycji agenta, dla alertów dotyczących agenta.
  - listy AI Agents, dla alertów dotyczących konta.

Link jest wstępnie uwierzytelniony, więc odbiorca jest o kliknięcie od podniesienia limitu lub wyłączenia agenta.

### Jak uruchamiane są progi

Platforma śledzi, które progi zostały już uruchomione w tym okresie, oddzielnie dla agenta i konta. Zatem:

- Przekroczenie `80%`, a następnie `100%` w tym samym okresie uruchamia oba, w kolejności.
- Przejście od 0% do `100%` w jednym dużym skoku uruchamia **najwyższy** przekroczony próg (`100%`), a nie `80%`, więc dostarczany jest najbardziej istotny alert.

### Kiedy przestajesz otrzymywać alerty

Jeżeli wydatki agenta nie osiągną następnego progu w tym okresie, nie otrzymasz kolejnych e-maili w tym okresie. Następny dzienny reset (lub miesięczny reset) czyści śledzenie.

### Wyłączanie alertów

Odznacz próg, którego nie chcesz. Jeśli nie chcesz żadnych alertów dla konkretnego agenta, odznacz wszystkie procenty. Alertów na poziomie konta nie można wyłączyć per agent (są one globalne dla konta).

### Zobacz także

- [Przegląd budżetów](#budgets-overview).
- [Powody odrzuceń](#drop-reasons) - co się dzieje, gdy limit zostanie całkowicie osiągnięty.
- [Model kosztów](#cost-model) - co jest mierzone.