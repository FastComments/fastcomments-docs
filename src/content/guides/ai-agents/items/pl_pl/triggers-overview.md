A **wyzwalacz** to zdarzenie, które wybudza agenta. Każdy agent może mieć zdefiniowany jeden lub więcej wyzwalaczy.

### The full list

| Wyzwalacz | Kiedy się uruchamia |
|---|---|
| [Dodano komentarz](#trigger-comment-add) | Dodano nowy komentarz. |
| [Edytowano komentarz](#trigger-comment-edit) | Komentarz został edytowany. Poprzedni tekst jest dołączony do kontekstu agenta. |
| [Usunięto komentarz](#trigger-comment-delete) | Komentarz został usunięty. |
| [Przypięto komentarz](#trigger-comment-pin) | Komentarz został przypięty (przez kogokolwiek, w tym moderatora lub innego agenta). |
| [Odpięto komentarz](#trigger-comment-unpin) | Komentarz został odpięty. |
| [Zablokowano komentarz](#trigger-comment-lock) | Komentarz został zablokowany (brak możliwości dalszych odpowiedzi). |
| [Odblokowano komentarz](#trigger-comment-unlock) | Komentarz został odblokowany. |
| [Komentarz przekracza próg głosów](#trigger-comment-vote-threshold) | Łączny wynik głosów komentarza osiąga skonfigurowany próg. |
| [Komentarz przekracza próg zgłoszeń](#trigger-comment-flag-threshold) | Liczba zgłoszeń komentarza osiąga dokładnie skonfigurowany próg. |
| [Użytkownik publikuje pierwszy komentarz](#trigger-new-user-first-comment) | Użytkownik publikuje swój pierwszy komentarz na tej witrynie. |
| [Komentarz oznaczony automatycznie jako spam](#trigger-comment-auto-spammed) | Komentarz jest automatycznie oznaczany jako spam przez silnik antyspamowy. |
| [Moderator oznacza komentarz jako sprawdzony](#trigger-moderator-reviewed) | Moderator oznacza komentarz jako sprawdzony. |
| [Moderator zatwierdza komentarz](#trigger-moderator-approved) | Moderator zatwierdza komentarz. |
| [Moderator oznacza komentarz jako spam](#trigger-moderator-spammed) | Moderator oznacza komentarz jako spam. |
| [Moderator przyznaje odznakę](#trigger-moderator-awarded-badge) | Moderator przyznaje użytkownikowi odznakę. |

### Multiple triggers per agent

Agent może subskrybować dowolne kombinacje wyzwalaczy - [Szablon moderatora](#template-moderator) subskrybuje na przykład zarówno `COMMENT_ADD`, jak i `COMMENT_FLAG_THRESHOLD`. Każde zdarzenie uruchamia agenta raz z kontekstem tego zdarzenia.

### What stops an agent firing

Subskrybowane zdarzenie wyzwalacza **nie** uruchamia agenta, jeśli zachodzi któreś z poniższych:

- Status agenta ([status](#status-states)) jest **Disabled**.
- Zakres [URL lub lokalizacji](#scope-url-locale) agenta nie pasuje do wyzwalającego komentarza.
- [Budżet dzienny, miesięczny lub limitu](#budgets-overview) agenta jest wyczerpany - wyzwalacz jest rejestrowany jako **dropped** z powodem. Zobacz [Przyczyny odrzuceń](#drop-reasons).
- Został osiągnięty limit współbieżności dla tego agenta (ograniczany dla każdego agenta).
- Tenant agenta ma nieprawidłowe rozliczenia.
- Działanie wyzwalające zostało wykonane przez bota lub innego agenta (zapobieganie pętli).
- Wyzwalacz dotyczył komentarza, który już został przetworzony przez tego agenta w oknie deduplikacji.

Kiedy subskrybowany wyzwalacz uruchamia się pomyślnie, [Historia uruchomień](#run-history) agenta pokazuje wiersz ze statusem **Started**, który przechodzi do **Success** lub **Error**, gdy uruchomienie się zakończy.

### Progi głosów i zgłoszeń

Dwa wyzwalacze - **Komentarz przekracza próg głosów** i **Komentarz przekracza próg zgłoszeń** - wymagają wartości progowej jako liczby na formularzu edycji. Wyzwalacz uruchamia się w momencie, gdy liczba przekracza skonfigurowaną wartość (konkretnie wyzwalacz progu zgłoszeń uruchamia się, gdy `flagCount === flagThreshold`, więc wybranie 1 oznacza "uruchom przy pierwszym zgłoszeniu", a wybranie 5 oznacza "uruchom, gdy nadejdzie piąte zgłoszenie").

### Odroczone wyzwalacze

Każdy wyzwalacz można odroczyć, aby agent uruchomił się później, na przykład po tym, jak głosy/zgłoszenia/odpowiedzi miały czas się ustabilizować. Zobacz [Odroczone wyzwalacze](#trigger-deferred-delay).

### Zapobieganie pętlom

Aby zapobiec nieskończonym pętlom, komentarze **utworzone przez agenta** zawierają `botId`. Wyzwalacze nowych komentarzy ignorują komentarze z `botId`.

Efekt końcowy: agenci mogą działać w odpowiedzi na *ludzkie* działania w Twoim tenantcie, ale działania pochodzące od agenta nigdy nie uruchamiają żadnych wyzwalaczy agenta. Dotyczy to wszystkich typów wyzwalaczy.

### REPLAY: the internal trigger

Istnieje także wewnętrzny typ wyzwalacza `REPLAY` używany przez funkcję [Uruchomienia testowe (Powtórki)](#test-runs-replays). Nie możesz go wybrać na formularzu edycji - istnieje po to, aby uruchomienia powtórkowe były wyraźnie oznaczane w historii uruchomień i wykluczane z widoków uruchomień na żywo.