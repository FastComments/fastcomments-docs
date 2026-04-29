Gdy agent umieszcza zatwierdzenie w kolejce, platforma powiadamia recenzentów za pomocą e‑maili. Dwa ustawienia na formularzu edycji kontrolują to: **kto** otrzymuje powiadomienia i **jak często**.

### Kto: tryb powiadamiania

Dwa tryby:

- **Wszyscy administratorzy i moderatorzy** (domyślnie) - każdy właściciel konta, superadministrator oraz administrator moderujący komentarze w tenancie jest potencjalnym recenzentem.
- **Konkretni użytkownicy** - wybierz ręcznie listę za pomocą kontrolki dwóch list na formularzu edycji.

W każdym przypadku potencjalny recenzent musi mieć konto w tenancie i ważny adres e‑mail, aby otrzymywać powiadomienia.

### Jak często: częstotliwość na użytkownika

Własny profil każdego potencjalnego recenzenta ustawia ich osobistą częstotliwość powiadomień o zatwierdzeniach agenta:

- **Natychmiastowe** (domyślnie) - jeden e‑mail na każde oczekujące zatwierdzenie, wysyłany zaraz po utworzeniu zatwierdzenia.
- **Co godzinę** - jeden zbiorczy e‑mail na godzinę podsumowujący wszystkie zatwierdzenia dodane w tej godzinie.
- **Codziennie** - jeden zbiorczy e‑mail co 24 godziny.
- **Wyłączone** - brak e‑maili. Użytkownik nadal może przeglądać zatwierdzenia przez interfejs skrzynki odbiorczej; po prostu nie otrzymuje powiadomień.

Użytkownik zmienia to ustawienie w swoim profilu, a nie na formularzu edycji agenta. To jest celowe - jeden tenant może mieć dziesięciu agentów, i moderator nie powinien musieć ustawiać preferowanej częstotliwości dla każdego agenta osobno.

### Zadania cron, które generują podsumowania

- **`hourly-agent-approval-digest`** - uruchamia się co godzinę, grupuje zatwierdzenia dodane od ostatniego podsumowania każdego użytkownika, wysyła jeden e‑mail na użytkownika.
- **`daily-agent-approval-digest`** - to samo, codziennie.
- **`agent-approval-reaper`** - usuwa zatwierdzenia, które mają więcej niż 90 dni, niezależnie od stanu.

Godzinne i dzienne zadania cron są ograniczone do odbiorcy: użytkownik z częstotliwością godzinową jest przetwarzany przez zadanie godzinne i pomijany przez zadanie dzienne (i odwrotnie). Użytkownicy z częstotliwością natychmiastową są powiadamiani przez ścieżkę tworzenia zatwierdzenia, a nie przez zadania cron.

### Stan deduplikacji

Platforma śledzi, którym użytkownikom wysłano już e‑maile dotyczące każdego zatwierdzenia. Gdy użytkownik zostanie powiadomiony (natychmiastowo lub w podsumowaniu), nie otrzyma ponownie e‑maila dotyczącego tego samego zatwierdzenia — nawet jeśli zmieni swoją częstotliwość z natychmiastowej na dzienną w trakcie cyklu.

### Zatwierdzanie z poziomu e‑maila

Każdy e‑mail powiadamiający zawiera jednoklikowy podpisany link logujący, który przenosi recenzenta bezpośrednio do strony ze szczegółami zatwierdzenia, już uwierzytelnionego. Może tam zatwierdzić, odrzucić lub otworzyć przepływ [Doprecyzowywanie podpowiedzi](#refining-prompts).

### Co jeśli nie ma administratorów

If `notifyMode` is `All admins and moderators` but the tenant has no super admins, comment moderator admins, or account owners with valid emails, the platform logs a warning and the approval still queues - just nobody gets notified about it. It will sit in the inbox until someone happens to look.

Jeśli `notifyMode` jest ustawione na `Specific users`, ale nie wybrano żadnych użytkowników, efekt jest ten sam.

### Co jeśli powiadomienia rozliczeniowe są wyłączone

[Alerty budżetowe](#budget-alerts) - e‑maile związane z budżetem - trafiają do administratorów rozliczeń **bez względu na osobiste ustawienia częstotliwości powiadomień**. Jest to zamierzone: przekroczenia budżetu wpływają na koszty, a właściciel rozliczeń musi o nich wiedzieć.

Powiadomienia o zatwierdzeniach respektują tylko ustawienie częstotliwości powiadomień o zatwierdzeniach agenta na poziomie użytkownika. Nie uwzględniają ogólnego wyłączenia powiadomień administratora - użytkownik, który zrezygnował z powiadomień administratora, nadal otrzyma e‑maile o zatwierdzeniach, jeśli jest na liście recenzentów, chyba że jego częstotliwość powiadomień o zatwierdzeniach agenta jest ustawiona na **Wyłączone**.

### Zobacz też

- [Przepływ zatwierdzania](#approval-workflow) dla pełnego cyklu życia zatwierdzenia.
- [Doprecyzowywanie podpowiedzi](#refining-prompts) dla przepływu "Ciągle zatwierdzam ten sam rodzaj błędu".