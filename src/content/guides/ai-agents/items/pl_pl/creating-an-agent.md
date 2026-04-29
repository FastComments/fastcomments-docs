Na stronie [Agentów AI](https://fastcomments.com/auth/my-account/ai-agents) możesz utworzyć agenta na dwa sposoby:

- **Z szablonu.** Kliknij **Przeglądaj szablony** i wybierz jednego z czterech wbudowanych agentów startowych. Formularz zostaje wstępnie wypełniony, a status agenta to **Tryb testowy**. Zobacz [Szablony startowe](#starter-templates).
- **Od zera.** Kliknij **Utwórz nowego agenta**. Formularz jest pusty.

Tak czy inaczej, ten sam formularz służy później do zapisu i edycji. Ta strona opisuje formularz od góry do dołu.

### Podstawy

- **Nazwa wewnętrzna.** Krótki identyfikator używany wyłącznie w panelach administracyjnych (historia uruchomień, analityka, logi audytu). Dobrze sprawdzają się małe litery z podkreśleniami: `moderator`, `welcome_greeter`. Jeśli nazwa wewnętrzna ze szablonu jest już zajęta, formularz automatycznie dopisuje sufiks (`tos_enforcer_2`, itp.).
- **Nazwa wyświetlana.** Pokazywana publicznie za każdym razem, gdy agent publikuje komentarz. To widzą Twoi czytelnicy.
- **Status.** Wyłączony, Tryb testowy lub Włączony. Nowe agenty domyślnie ustawione są na Tryb testowy. Zobacz [Stany statusu](#status-states).

### Model

Wybierz LLM. Zobacz [Wybór modelu](#choosing-a-model).

### Budżet

Opcjonalne dzienne i miesięczne limity w walucie konta, oraz lista kontrolna **Progi alertów** (domyślnie 80% i 100%). Zobacz [Przegląd budżetów](#budgets-overview) oraz [Alerty budżetowe](#budget-alerts).

### Osobowość

**Początkowy prompt** to systemowy prompt definiujący ton, rolę i reguły decyzyjne. Zwykły tekst, bez składni szablonów. Zobacz [Osobowość i początkowy prompt](#personality-prompt).

### Kontekst

Sekcja Kontekst zawiera trzy pola wyboru, pole tekstowe z wytycznymi oraz pola zakresu:

- Dołącz komentarz nadrzędny i wcześniejsze odpowiedzi w tej samej wątku.
- Dołącz współczynnik zaufania komentującego, wiek konta, historię banów i ostatnie komentarze.
- Dołącz tytuł strony, podtytuł, opis i meta tagi.
- Opcjonalny blok tekstu **Wytyczne społeczności**, który jest dodawany przed każdym promptem.
- **Ogranicz do konkretnych stron** - lista dozwolonych wzorców URL (po jednym w wierszu). Puste oznacza zastosowanie do całego najemcy.
- **Ogranicz do konkretnych lokalizacji** - lista dozwolonych lokalizacji wybierana za pomocą podwójnego selektora. Puste oznacza wszystkie lokalizacje.

Większy kontekst poprawia jakość decyzji, ale zwiększa koszt tokenów na jedno uruchomienie. Zobacz [Opcje kontekstu](#context-options), [Wytyczne społeczności](#community-guidelines) oraz [Zakres: filtry URL i lokalizacji](#scope-url-locale).

### Wyzwalacze

Wybierz co najmniej jedno zdarzenie z listy. Dla wyzwalaczy progów głosów i progów flag musisz również ustawić próg. Opcjonalne pole **Opóźnienie przed uruchomieniem** odkłada wykonanie po wyzwoleniu (przydatne dla progów flag, gdzie głosy wciąż się stabilizują). Zobacz [Przegląd zdarzeń wyzwalających](#triggers-overview) oraz [Opóźnione wyzwalacze](#trigger-deferred-delay).

### Dozwolone wywołania narzędzi

Zaznacz **Zezwól na dowolne wywołania narzędzi**, aby udostępnić pełne palety narzędzi. W przeciwnym razie zaznacz konkretne narzędzia, do których agent ma mieć dostęp — niedozwolone narzędzia są usuwane z palety modelu i odrzucane przy dispatchu. Podsekcja **Opcje banów** blokuje destrukcyjne warianty banów (delete-all-comments, ban-by-IP) za wyraźną zgodą. Zobacz [Przegląd dozwolonych wywołań narzędzi](#tools-overview) oraz [Narzędzie: ban_user](#tool-ban-user).

### Zatwierdzenia

Zaznacz akcje, które muszą zostać zatwierdzone przez człowieka, zanim agent je wykona. Zatwierdzenia dotyczą tylko narzędzi, do których agent ma prawo wywołać; niedozwolone narzędzia są odrzucane wprost. W regionie UE, **ban_user** jest włączone z mocy artykułu 17 Digital Services Act. Zobacz [Proces zatwierdzania](#approval-workflow) oraz [Zgodność z art. 17 DSA UE](#eu-dsa-compliance).

### Powiadomienia o zatwierdzeniach

Jeśli zatwierdzenia są włączone, wybierz, kto otrzymuje e‑mail:

- **Wszyscy administratorzy i moderatorzy** - właściciele konta, superadmini i administratorzy moderatorów komentarzy.
- **Konkretni użytkownicy** - wybrani ręcznie za pomocą podwójnego selektora.

Indywidualna częstotliwość dostarczania powiadomień każdego recenzenta (natychmiast, podsumowanie godzinne, podsumowanie dzienne) jest ustawiana w jego profilu. Zobacz [Powiadomienia o zatwierdzeniach](#approval-notifications).

### Statystyki

Tylko do odczytu. Łączna liczba uruchomień, znacznik czasu ostatniego uruchomienia oraz identyfikator najnowszego komentarza napisanego przez agenta (jeśli istnieje).

### Zapisz

Kliknij **Zapisz agenta**. Strona przekierowuje z powrotem do listy agentów. Nowe agenty od razu są uprawnione do otrzymywania wyzwalaczy w trybie testowym.

### Edycja później

Każdy wiersz na stronie listy agentów udostępnia akcje per-agent: **Edytuj**, **Klonuj**, **Uruchomienia**, **Odtworzenia**, **Uruchomienie testowe**, **Analityka**, **Zatwierdzenia** i **Usuń**. Edycja agenta nie wpływa retrospektywnie na już zarejestrowane uruchomienia — historia jest zachowana. Migawki odtworzeń również zamrażają konfigurację agenta w momencie rozpoczęcia odtworzenia, dzięki czemu wyniki zapisanego odtworzenia pozostają powtarzalne nawet po edycji promptu.