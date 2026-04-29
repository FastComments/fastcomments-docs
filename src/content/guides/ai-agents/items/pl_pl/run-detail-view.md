Kliknięcie **View** w wierszu w [Historia uruchomień](#run-history) otwiera stronę szczegółów pojedynczego uruchomienia. To tutaj czytasz rozumowanie agenta i oceniasz jego decyzje.

### Top: podsumowanie uruchomienia

- **Agent** - który agent wykonał uruchomienie.
- **When** - znacznik czasu.
- **Status** - Rozpoczęto / Sukces / Błąd, oraz odznaka **Tryb testowy** jeśli dotyczy.
- **Cost** - koszt pojedynczego uruchomienia w walucie Twojego najemcy.
- **Cost per action** - koszt podzielony przez liczbę działań nieoczekujących, przydatne do wykrywania wyjątkowo kosztownych uruchomień.

### Wykonane działania

Lista, w kolejności, wszystkich wywołań narzędzi wykonanych przez uruchomienie. Każdy wpis pokazuje:

- **Action label** - "Wszystko: napisał komentarz", "Oznaczył komentarz jako spam", "Zbanował użytkownika" i tak dalej. Etykieta mapuje się z typem akcji w enumie.
- **Reference ID** - dotknięte ID komentarza, użytkownika lub odznaki, pokazane jako tekst w stałej szerokości czcionki (nie jako link).
- **Agent reasoning** - uzasadnienie podane przez agenta przy wywołaniu.
- **Confidence** - samoocena pewności agenta, wyświetlana jako procent.
- **Pending approval** badge - jeśli akcja jest w kolejce w [skrzynce zatwierdzeń](#approval-workflow) zamiast być wykonana.

Jeśli uruchomienie nie wykonało żadnych działań, sekcja zawiera tekst: "No actions were taken during this run."

### Transkrypt LLM

Poniżej działań znajduje się pełny transkrypt rozmowy agenta z LLM:

- **System** - systemowy prompt (sufiks platformy + Twój początkowy monit + wytyczne społeczności).
- **User** - wiadomość kontekstowa opisująca wyzwalacz.
- **Assistant** - odpowiedzi modelu, w tym wywołania narzędzi.
- **Tool** - wynik narzędzia przekazany z powrotem do modelu (np. to, co zwróciło `search_memory`).

Długie wiadomości można zwijać; kliknij **Expand** / **Collapse**, aby wyświetlić.

### Czytanie transkryptów

Transkrypt to najważniejsza strona do strojenia. Gdy agent podejmie decyzję, z którą się nie zgadzasz, przeczytaj go, aby zobaczyć:

- Co model **widział** (wiadomość kontekstowa User).
- Co model **zdecydował** (wywołania narzędzi Assistant).
- Co model **rozważał** (wszystkie wyniki narzędzi - np. czy agent faktycznie wywołał `search_memory` i czy znalazł coś przed zbanowaniem).

Jeśli model stale popełnia ten sam rodzaj błędu, edytuj [początkowy monit](#personality-prompt) - lub użyj [Doprecyzowywania promptów](#refining-prompts) z odrzuconego zatwierdzenia.

### Odniesienia do akcji

ID referencyjne są pokazywane jako tekst stałej szerokości (nie jako linki):

- Komentarze: ID komentarza.
- Użytkownicy: ID użytkownika.
- Odznaki: ID odznaki.

Możesz skopiować ID, aby wyszukać dotknięty rekord na odpowiedniej stronie moderacji/admina.

### Czego brakuje w trybie testowym

Uruchomienia w trybie testowym pokazują **te same** działania, uzasadnienia i wyniki pewności. Jedyną różnicą jest odznaka **Tryb testowy** w wierszu statusu. ID referencyjne komentarzy / użytkowników / odznak są nadal wyświetlane — agent ich po prostu nie zmodyfikował.

### Błędy

Dla uruchomień w stanie `Error`, strona szczegółów pokazuje podstawowy komunikat o błędzie. Typowe błędy:

- **No LLM API key configured** - nieprawidłowa konfiguracja najemcy lub platformy.
- **LLM call timeout** - dostawca LLM był wolny lub nieosiągalny.
- **Tool dispatch failure** - agent wybrał narzędzie z błędnymi argumentami (np. ID komentarza, które już nie istnieje).
- **Budget exhausted mid-run** - limit agenta został osiągnięty w trakcie uruchomienia. Uruchomienie zostało zatrzymane.

Błędy nie cofają częściowych działań - wszelkie wywołania narzędzi zakończone przed wystąpieniem błędu pozostają wykonane.