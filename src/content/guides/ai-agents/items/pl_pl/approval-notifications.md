Gdy agent kolejkuje zatwierdzenie, platforma powiadamia recenzentów e-mailem. Dwa ustawienia w formularzu edycji to kontrolują: **kto** jest powiadamiany i **jak często**.

### Kto: tryb powiadamiania

Dwa tryby:

- **Wszyscy administratorzy i moderatorzy** (domyślnie) - każdy właściciel konta, super admin i administrator moderatora komentarzy w ramach tenanta jest potencjalnym recenzentem.
- **Konkretni użytkownicy** - ręcznie wybierz listę za pomocą podwójnej listy w formularzu edycji.

W obu przypadkach potencjalny recenzent musi mieć konto w ramach tenanta oraz ważny adres e-mail, aby otrzymywać powiadomienia.

### Jak często: częstotliwość per użytkownik

**Własny profil** każdego potencjalnego recenzenta ustawia jego osobistą częstotliwość powiadomień o zatwierdzeniach agenta:

- **Natychmiast** (domyślnie) - jeden e-mail na oczekujące zatwierdzenie, wysyłany natychmiast po jego utworzeniu.
- **Co godzinę** - jeden e-mail zbiorczy na godzinę, podsumowujący wszystkie zatwierdzenia zakolejkowane w tej godzinie.
- **Codziennie** - jeden e-mail zbiorczy na 24 godziny.
- **Wyłączone** - brak e-maili. Użytkownik nadal może przeglądać zatwierdzenia w interfejsie skrzynki odbiorczej; po prostu nie jest powiadamiany.

Użytkownik zmienia to ustawienie na własnym profilu, a nie w formularzu edycji agenta. To celowe - jeden tenant może mieć dziesięć agentów, a moderator nie powinien musieć ustawiać preferowanej częstotliwości osobno dla każdego agenta.

### Zadania cron sterujące e-mailami zbiorczymi

- **`hourly-agent-approval-digest`** - uruchamia się co godzinę, grupuje zatwierdzenia zakolejkowane od ostatniego e-maila zbiorczego danego użytkownika i wysyła jeden e-mail na użytkownika.
- **`daily-agent-approval-digest`** - to samo, codziennie.
- **`agent-approval-reaper`** - usuwa zatwierdzenia, które przekroczyły 90 dni niezależnie od stanu.

Zadania cron godzinowe i dzienne są ograniczone do odbiorcy: użytkownik z częstotliwością godzinową jest obsługiwany przez zadanie godzinowe i pomijany przez dzienne (i odwrotnie). Użytkownicy z częstotliwością natychmiastową są powiadamiani w ścieżce kodu tworzącej zatwierdzenie, a nie przez zadania cron.

### Stan deduplikacji

Platforma śledzi, którzy użytkownicy otrzymali już e-mail dotyczący danego zatwierdzenia. Gdy użytkownik został powiadomiony (natychmiast lub w e-mailu zbiorczym), nie otrzyma ponownie e-maila o tym samym zatwierdzeniu - nawet jeśli w trakcie cyklu zmieni częstotliwość z natychmiastowej na dzienną.

### Zatwierdzanie z e-maila

Każdy e-mail z powiadomieniem zawiera jednorazowy podpisany link logowania, który prowadzi recenzenta bezpośrednio do strony szczegółów zatwierdzenia, już zalogowanego. Stamtąd może zatwierdzić, odrzucić lub otworzyć przepływ [Refine Prompts](#refining-prompts).

### Co jeśli nie ma administratorów

Jeśli `notifyMode` to `All admins and moderators`, ale tenant nie ma super adminów, administratorów moderatorów komentarzy ani właścicieli kont z ważnymi adresami e-mail, platforma rejestruje ostrzeżenie, a zatwierdzenie i tak trafia do kolejki - po prostu nikt nie zostanie o nim powiadomiony. Będzie czekać w skrzynce odbiorczej, dopóki ktoś przypadkiem tam nie zajrzy.

Jeśli `notifyMode` to `Specific users`, ale nie wybrano żadnych użytkowników, wynik jest taki sam.

### Co jeśli powiadomienia rozliczeniowe są wyłączone

[Budget Alerts](#budget-alerts) - e-maile dotyczące budżetu - trafiają do administratorów rozliczeń **niezależnie od osobistych preferencji powiadomień użytkownika**. To celowe: przekroczenia budżetu wpływają na koszty, a właściciel rozliczeń musi o tym wiedzieć.

Powiadomienia o zatwierdzeniach honorują wyłącznie ustawienie częstotliwości zatwierdzeń agenta per użytkownik. Nie sprawdzają one szerszej opcji rezygnacji z powiadomień administratora - użytkownik, który zrezygnował z powiadomień administratora, nadal będzie otrzymywał e-maile o zatwierdzeniach, jeśli znajduje się na liście recenzentów, chyba że jego częstotliwość zatwierdzeń agenta jest ustawiona na **Wyłączone**.

### Zobacz także

- [Approval Workflow](#approval-workflow) - pełny cykl życia zatwierdzenia.
- [Refining Prompts](#refining-prompts) - przepływ "ciągle zatwierdzam ten sam rodzaj błędu".