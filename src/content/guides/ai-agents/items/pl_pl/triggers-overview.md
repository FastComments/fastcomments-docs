**Wyzwalacz (trigger)** to zdarzenie, które budzi agenta. Każdy agent może mieć zdefiniowany jeden lub więcej wyzwalaczy.

### Pełna lista

| Wyzwalacz | Kiedy się uruchamia |
|---|---|
| [Comment Added](#trigger-comment-add) | Opublikowano nowy komentarz. |
| [Comment Edited](#trigger-comment-edit) | Komentarz został edytowany. Poprzednia treść jest dołączona do kontekstu agenta. |
| [Comment Deleted](#trigger-comment-delete) | Komentarz został usunięty. |
| [Comment Pinned](#trigger-comment-pin) | Komentarz został przypięty (przez kogokolwiek, w tym moderatora lub innego agenta). |
| [Comment Unpinned](#trigger-comment-unpin) | Komentarz został odpięty. |
| [Comment Locked](#trigger-comment-lock) | Komentarz został zablokowany (nie można dodawać kolejnych odpowiedzi). |
| [Comment Unlocked](#trigger-comment-unlock) | Komentarz został odblokowany. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | Netto głosów komentarza osiąga skonfigurowany próg. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | Liczba zgłoszeń (flag) komentarza osiąga dokładnie skonfigurowany próg. |
| [User Posts First Comment](#trigger-new-user-first-comment) | Użytkownik publikuje swój pierwszy komentarz na tej stronie. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | Komentarz został automatycznie oznaczony jako spam przez silnik antyspamowy. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | Moderator oznacza komentarz jako sprawdzony. |
| [Moderator Approves Comment](#trigger-moderator-approved) | Moderator zatwierdza komentarz. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | Moderator oznacza komentarz jako spam. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | Moderator przyznaje użytkownikowi odznakę. |

### Wiele wyzwalaczy na agenta

Agent może subskrybować dowolną kombinację wyzwalaczy - na przykład [szablon Moderator](#template-moderator) subskrybuje zarówno `COMMENT_ADD`, jak i `COMMENT_FLAG_THRESHOLD`. Każde zdarzenie uruchamia agenta raz, z kontekstem tego zdarzenia.

### Co uniemożliwia uruchomienie agenta

Zasubskrybowane zdarzenie wyzwalacza **nie** uruchamia agenta, jeśli spełniony jest którykolwiek z poniższych warunków:

- [Status](#status-states) agenta to **Disabled**.
- [Zakres URL lub locale](#scope-url-locale) agenta nie pasuje do komentarza wywołującego.
- [Dzienny, miesięczny lub limit-rate budżet](#budgets-overview) agenta jest wyczerpany - wyzwalacz jest rejestrowany jako **dropped** z powodem. Zobacz [Drop Reasons](#drop-reasons).
- Współbieżność dla tego agenta jest wyczerpana (limit na agenta).
- Najemca (tenant) agenta ma nieprawidłowe rozliczenia.
- Wywołująca akcja została sama wykonana przez bota lub innego agenta (zapobieganie pętlom).
- Wyzwalacz dotyczył komentarza, który został już przetworzony przez tego agenta w oknie deduplikacji.

Gdy zasubskrybowany wyzwalacz uruchomi się pomyślnie, [Run History](#run-history) agenta pokazuje wiersz ze statusem **Started**, który przechodzi w **Success** lub **Error** po zakończeniu uruchomienia.

### Progi głosów i zgłoszeń

Dwa wyzwalacze - **Comment Crosses Vote Threshold** i **Comment Crosses Flag Threshold** - wymagają liczbowego progu w formularzu edycji. Wyzwalacz uruchamia się w momencie przekroczenia skonfigurowanej wartości (konkretnie, wyzwalacz progu zgłoszeń uruchamia się, gdy `flagCount === flagThreshold`, więc wybranie 1 oznacza "uruchom przy pierwszym zgłoszeniu", a wybranie 5 oznacza "uruchom, gdy przyjdzie piąte zgłoszenie").

### Wyzwalacze odroczone

Każdy wyzwalacz może być odroczony, tak aby agent uruchomił się później, na przykład gdy głosy/zgłoszenia/odpowiedzi zdążą się ustabilizować. Zobacz [Deferred Triggers](#trigger-deferred-delay).

### Zapobieganie pętlom

Aby zapobiec nieskończonym pętlom, komentarze **napisane przez agenta** zawierają `botId`. Wyzwalacze nowych komentarzy ignorują komentarze z `botId`.

Efekt netto: agenty mogą reagować na działania *ludzi* w Twoim tenancie, ale działania pochodzące od agentów nigdy nie uruchamiają wyzwalaczy żadnego agenta. Dotyczy to
wszystkich typów wyzwalaczy.

### REPLAY: wewnętrzny wyzwalacz

Istnieje również wewnętrzny typ wyzwalacza `REPLAY`, używany przez funkcję [Test Runs (Replays)](#test-runs-replays). Nie można go wybrać w formularzu edycji - istnieje po to, aby uruchomienia replay były wyraźnie oznaczone w historii uruchomień i wykluczone z widoków uruchomień na żywo.