Agent memory jest pulą klucz‑wartość o zakresie najemcy, **współdzieloną**, do której każdy agent w Twoim najemcy może odczytywać i zapisywać. Istnieje, aby agenci mogli przenosić kontekst pomiędzy uruchomieniami.

### Dlaczego istnieje pamięć

Kontekst LLM jest określany na każde uruchomienie. Bez pamięci agent, który wydaje ostrzeżenie użytkownikowi, nie ma możliwości poznania tego ostrzeżenia przy następnym spotkaniu z tym samym użytkownikiem. Polityka eskalacji platformy – „ostrzeż przed zbanowaniem” – zależy od tego, że agent może odnaleźć poprzednie ostrzeżenie. Pamięć jest tym, co to umożliwia.

### Dwa rodzaje pamięci

- **WARNING** – zapisywane automatycznie jako część przepływu [`warn_user`](#tool-warn-user). Agent nie zapisuje rekordów `WARNING` ręcznie; są one skutkiem ubocznym ostrzegania użytkownika.
- **NOTE** – zapisywane przez [`save_memory`](#tools-overview). Ogólny kontekst, który agent chce, aby przyszłe agenty znały.

Polityka eskalacji szuka konkretnie rekordów `WARNING` przy decydowaniu, czy ban jest uzasadniony.

### Zakres najemcy, współdzielona przez agenta

Wszyscy agenci w Twoim najemcy współdzielą **jedną pulę pamięci**. Notatka zapisana przez Agenta A jest widoczna w wywołaniach `search_memory` Agenta B. Jest to zamierzone – chcesz, aby notatki agenta triage informowały decyzje agenta moderatora.

`tenantId` jest ustawiane przez wykonawcę z własnego najemcy agenta – nigdy z argumentów LLM – więc wycieki pamięci między najemcami są niemożliwe z konstrukcji.

### Co znajduje się w rekordzie pamięci

Każdy wpis pamięci zawiera:

- **Który agent go zapisał**, oraz kiedy.
- **Kogo dotyczy** – użytkownika, którego opisuje ta pamięć. Agent nie może tego wymyślić; platforma wypełnia to automatycznie na podstawie tego, co wywołało agenta.
- **Ukryty sygnał alternatywnego konta** – platforma również rejestruje (prywatnie) odcisk palca IP pochodzącego komentarza, aby przyszłe wyszukiwania pamięci mogły wyświetlać notatki o innych kontach publikujących z tego samego IP. Odcisk palca nigdy nie jest pokazywany agentowi ani LLM.
- **Sama notatka** – do 2000 znaków wolnego tekstu.
- **Tagi** do wyszukiwania – do 10 krótkich tagów.
- **Rodzaj** – albo ostrzeżenie, albo ogólna notatka.
- **Opcjonalny link do komentarza** – jeśli pamięć jest powiązana z konkretnym komentarzem.

### Zachowanie wyszukiwania

[`search_memory`](#tools-overview) zwraca maksymalnie 25 rekordów, posortowanych od najnowszych, automatycznie ograniczonych do (użytkownika wyzwalacza) LUB (innych kont na IP wyzwalacza). Wyniki są również ograniczone do 8000 znaków łącznej treści wszystkich zwróconych elementów – starsze wpisy są pomijane, jeśli limit zostanie przekroczony.

Agent nie przekazuje `userId` ani `targetIpHash`. Oba są ustawiane przez wykonawcę.

### Trwałość

Pamięć ma **brak TTL**. Rekordy pozostają, dopóki nie zostaną wyraźnie usunięte. Rekordy WARNING dotyczące użytkownika są celowo nigdy nie usuwane automatycznie – historia eskalacji musi być dostępna bezterminowo, w przeciwnym razie sprawdzenie platformy „wyszukaj przed zbanowaniem” jest bez sensu.

Trzy sposoby usuwania pamięci:

- Moderator usuwa podstawowy komentarz – każda pamięć powiązana z tym komentarzem jest kaskadowo usuwana.
- Użytkownik zostaje usunięty – wszystkie wpisy pamięci dotyczące tego użytkownika są usuwane w tej samej transakcji.
- Twój najemca zostaje usunięty.

Obecnie nie ma interfejsu administracyjnego do usuwania pojedynczych rekordów pamięci.

### Pamięć w trybie dry-run

Agenci w trybie dry-run **nie** zapisują pamięci. Jest to zamierzone: hipotetyczne decyzje agenta dry-run nie powinny zanieczyszczać współdzielonej puli pamięci. Odczyt za pomocą `search_memory` działa w trybie dry-run normalnie – agent może zobaczyć rzeczywiste pamięci od żywych agentów – po prostu nie może ich dodawać.

### Pamięć w powtórkach (replays)

Tak samo jak dry-run: agenci powtórek nie zapisują pamięci. Powtórki są tylko podglądem. Zobacz [Test Runs (Replays)](#test-runs-replays).

### Podsumowanie ograniczeń

| Limit | Wartość |
|---|---|
| Maksymalna długość treści pamięci | 2000 znaków |
| Maksymalna długość tagu pamięci | 64 znaki |
| Maksymalna liczba tagów pamięci | 10 |
| Maksymalna długość zapytania pamięci | 200 znaków |
| Limit wyników wyszukiwania pamięci | 25 rekordów |
| Całkowity limit treści wyników wyszukiwania pamięci | 8000 znaków |

### Zobacz także

- [Tool: save_memory](#tools-overview) do zapisu.
- [Tool: search_memory](#tools-overview) do odczytu.
- [Tool: warn_user](#tool-warn-user) – jedyne narzędzie, które zapisuje pamięć rodzaju WARNING.
- [Tool: ban_user](#tool-ban-user) – systemowy prompt wymaga wywołania `search_memory` przed tym.