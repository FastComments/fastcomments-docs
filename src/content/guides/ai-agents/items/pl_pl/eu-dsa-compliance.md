FastComments egzekwuje Artykuł 17 rozporządzenia UE o usługach cyfrowych (Digital Services Act) dla tenantów w regionie UE: **w pełni zautomatyzowane zawieszenia użytkowników nie są dozwolone**.

### Co to oznacza w praktyce

Gdy Twój tenant znajduje się w regionie UE, na formularzu edycji agenta:

- Pole wyboru **Approvals** dla `ban_user` jest **zablokowane w pozycji włączonej** i nie można go odznaczyć.
- Etykieta brzmi: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- Dymek pomocy na kolumnie zatwierdzeń brzmi: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Cokolwiek innego skonfigurujesz, każde wywołanie `ban_user` od dowolnego agenta na tenancie w regionie UE trafia do [approvals inbox](#approval-workflow) do przeglądu przez człowieka. Blokada nie zostanie nałożona, dopóki człowiek jej nie zatwierdzi.

### Dlaczego jest to egzekwowane na poziomie platformy, a nie promptu

Systemowe prompty mogą zostać zignorowane lub obejści przez wystarczająco źle działający model. Zgodność z Artykułem 17 jest zbyt ważna, aby polegać na dobrym zachowaniu modelu; musi to być twarda bramka po stronie serwera, którą sam dispatcher narzędzi egzekwuje. I to właśnie robimy.

### Co przechodzi, a co nie przechodzi przez zatwierdzenie

- **`ban_user`**: zawsze objęte blokadą w UE. W tym:
  - Widoczne bany (`shadowBan: false`).
  - Ciche blokady (`shadowBan: true`).
  - Bany z `deleteAllUsersComments: true`.
  - Bany z `banIP: true`.
- Wszystkie warianty blokad trafiają do skrzynki zatwierdzeń wraz z uzasadnieniem i pewnością agenta; człowiek zatwierdza lub odrzuca.

Inne narzędzia agenta (`mark_comment_spam`, `warn_user`, `lock_comment` itd.) **nie** są objęte Artykułem 17. Nadal możesz je automatyzować. Artykuł 17 dotyczy konkretnie zawieszeń użytkowników.

### A co z tenantami spoza UE

Blokada nie obowiązuje poza regionem UE. Możesz jednak samodzielnie zdecydować o umieszczeniu `ban_user` za zatwierdzeniem — zdecydowanie zalecamy to w pierwszych tygodniach działania każdego agenta moderacji — ale nie jest to narzucone.

### Ciche blokady

Ciche blokady są liczone jako zawieszenia w rozumieniu Artykułu 17 (użytkownik może publikować, ale jego treść jest ukryta). Są objęte tą samą blokadą co widoczne bany.

### Wykrywanie regionu

Region jest określany na poziomie procesu przez zmienną środowiskową `REGION` na wdrożeniu FastComments (odczytywaną przez `isEURegion()` w `models/constants.ts`). Nie ma pola regionu per tenant — blokada dotyczy każdego tenanta na instancji wdrożonej w UE. Jeśli przeniesiesz swoje dane z wdrożenia spoza UE do wdrożenia w UE, blokada zacznie obowiązywać dla wszystkich tenantów na tej instancji.

### Co jeśli wszyscy recenzenci są niedostępni

Zatwierdzenie będzie czekać w skrzynce, aż zostanie podjęta decyzja. Automatycznie wygaśnie 90 dni po utworzeniu. Nie ma ścieżki „brak dostępnego recenzenta, przejście do automatycznej decyzji” — to podważałoby sens Artykułu 17.

Jeśli Twoja społeczność jest tak duża, że bany w UE nie mogą być przeglądane w rozsądnym czasie, rozważ:

- Dodanie większej liczby recenzentów (zobacz [Approval Notifications](#approval-notifications)).
- Przełączenie agenta na bardziej agresywne używanie [`warn_user`](#tool-warn-user), ponieważ ostrzeżenia nie podlegają Artykułowi 17.
- Zmniejszenie skłonności agenta do banowania przez zaostrzenie [community guidelines](#community-guidelines) lub [initial prompt](#personality-prompt).

### Zobacz też

- [Tool: ban_user](#tool-ban-user) aby dowiedzieć się, co robi `ban_user` i jakie destrukcyjne opcje są dostępne po dodatkowych opt-inach.
- [Approval Workflow](#approval-workflow) aby poznać pełny cykl życia zatwierdzeń.