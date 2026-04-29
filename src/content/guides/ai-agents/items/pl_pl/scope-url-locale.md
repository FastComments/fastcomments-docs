Domyślnie agent działa w ramach całego tenanta — na każdej stronie, w każdej lokalizacji. Sekcje **Zakres** i **Lokalizacje** w formularzu edycji pozwalają to zawęzić.

### Ogranicz do określonych stron

Pole **Restrict to specific pages** akceptuje jeden wzorzec URL na linię, w składni url-pattern glob. Agent uruchamia się tylko dla komentarzy, których URL strony pasuje do przynajmniej jednego z wzorców. Przykłady:

- `/news/*` - każda strona pod `/news`.
- `/forums/*` - każda strona pod `/forums`.
- `/blog/2026/*` - każda strona pod `/blog/2026`.
- (wiele linii razem) - agent uruchamia się, jeśli **którykolwiek** wzorzec pasuje.

Maksimum: 50 wzorców na agenta. Wzorce muszą być poprawnymi url-pattern globami - formularz odrzuca błędne z konkretnym komunikatem o błędzie.

Gdy pole jest **puste**, agent działa na każdej stronie w tenancie.

Gdy pole jest **niepuste**, agent zachowuje się zamknięcie-bezpieczeństwo: każdy wyzwalacz, którego komentarz nie ma `urlId` (np. zdarzenia na poziomie tenanta bez kontekstu strony), jest pomijany. Tak jest zaprojektowane — "ograniczone do /news/*" nie powinno cicho przechodzić do "wszystkiego".

### Ogranicz do określonych lokalizacji

Kontrolka **Restrict to specific locales** z dwoma listami akceptuje identyfikatory lokalizacji FastComments (`en_us`, `zh_cn`, `de_de` itd.). Agent uruchamia się tylko dla komentarzy, których wykryta lokalizacja znajduje się na wybranej liście.

Wykryta lokalizacja pochodzi z pola `locale` komentarza, które jest ustawiane przez widget komentarzy w momencie publikacji na podstawie lokalizacji strony.

Gdy **żadne lokalizacje** nie są wybrane, agent działa we wszystkich lokalizacjach.

Gdy wybrana jest **jedna lub więcej lokalizacji**, agent zachowuje się zamknięcie-bezpieczeństwo: wyzwalacze bez komentarza, lub wyzwalacze dotyczące komentarzy bez pola `locale`, są pomijane.

### Połączone ograniczenia

Filtry URL i lokalizacji łączą się poprzez AND. Wyzwalacz uruchamia agenta tylko wtedy, gdy **oba** filtry na to pozwalają.

Przydatne wzorce:
- `/news/*` wzorzec URL + `en_us` lokalizacja - tylko angielska sekcja wiadomości.
- Brak filtra URL + wiele lokalizacji - obejmuje cały tenant, ale tylko języki, dla których napisano prompt tego agenta.

### Dlaczego ograniczać agenta

- **Koszt.** Ograniczenie zakresu zmniejsza liczbę wyzwalaczy, które agent musi przetworzyć, a tym samym redukuje wydatki na tokeny.
- **Poprawność.** Prompt podsumowujący dostrojony pod artykuły techniczne może dawać słabe wyniki na stronach produktowych. Ograniczenie zakresu to skuteczniejsze narzędzie niż proszenie promptu, by „pomiń strony nietechniczne” po angielsku.
- **Zachowanie specyficzne dla lokalizacji.** Powitanie, które pisze tylko po niemiecku, powinno uruchamiać się jedynie dla komentarzy z niemiecką lokalizacją. Połącz zakres `de_de` z niemieckojęzycznym tonem w [początkowym promptzie](#personality-prompt).

### Czego ograniczenie nie robi

- Nie zmienia **liczby slotów agenta** (zob. [Plany i uprawnienia](#plans-and-eligibility)) — ograniczony agent nadal zajmuje jedno miejsce.
- Nie zmienia [limitów budżetowych](#budgets-overview) — dzienne i miesięczne limity na agenta obowiązują dla wszystkich pasujących wyzwalaczy.
- Nie działa retroaktywnie wobec poprzednich uruchomień — historia uruchomień pokazuje wszystko, co agent zrobił, nawet jeśli później zawęzisz jego zakres.