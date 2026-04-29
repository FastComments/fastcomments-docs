Każdy agent działa w oparciu o jeden z dwóch modeli LLM, wybierany w sekcji **Model** formularza edycji.

### Dwie opcje

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - domyślny. Wyższa jakość rozumowania, nieco wolniejszy przy każdym wywołaniu. Zalecany dla agentów w stylu moderacji (`Moderator` template, anything that calls `ban_user` or `mark_comment_spam`) gdzie koszt błędnego wywołania jest wysoki.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - szybszy przy wywołaniu, niższa latencja. Zalecany dla agentów o dużym wolumenie i niskiej stawce (powitalny agent, przypinacz wątków), gdzie chcesz otrzymywać odpowiedzi w ciągu sekund, a konsekwencje błędnego wywołania są niewielkie.

Oba modele obsługują wywoływanie funkcji, oba działają za pośrednictwem tego samego OpenAI-compatible API, i oba korzystają z tych samych schematów dla poszczególnych narzędzi - więc możesz przełączać zapisany agent między nimi w dowolnym momencie bez innych zmian konfiguracji.

### Różnice w kosztach

Dwa modele mają różne koszty za token. [Limity budżetowe](#budgets-overview) agenta są denominowane w walucie Twojego konta, a nie w tokenach, więc przełączenie z jednego modelu na drugi zmienia, ile uruchomień mieści się w Twoich dziennych i miesięcznych limitach. Strona [Run History](#run-history) pokazuje koszt za uruchomienie w Twojej walucie po zakończeniu uruchomienia - obserwowanie kilku uruchomień po zmianie to najprostszy sposób, by ocenić nową szybkość spalania środków.

### Tokeny na uruchomienie

Wykorzystanie tokenów przez odpowiedź modelu jest rejestrowane przy każdym wyzwalaczu jako **tokensUsed**. Jest ono zawarte w ładunkach webhooków `trigger.succeeded` i `trigger.failed` (zob. [Webhook Payloads](#webhook-payloads)) oraz pokazane w [Run Detail View](#run-detail-view). Ilość ta zależy od:

- Ile [Kontekstu](#context-options) uwzględnisz - kontekst wątku, historia użytkownika i metadane strony wszystkie dodają tokeny.
- Jak długi jest Twój [Początkowy prompt](#personality-prompt) i [Zasady społeczności](#community-guidelines).
- Ile narzędzi agent wywołuje w jednym uruchomieniu (każde wywołanie narzędzia i jego wynik przechodzą tam i z powrotem przez model).

**Maksymalna liczba tokenów na wyzwalacz** (domyślnie 20,000) jest górnym limitem na uruchomienie, ustawianym dla każdego agenta.

### Zmiana modeli

Możesz zmienić model w formularzu edycji w dowolnym momencie. Istniejąca historia uruchomień i analizy zachowują swoje oryginalne liczby tokenów i kosztów - są one rejestrowane w czasie uruchamiania. Nowy model ma zastosowanie tylko do uruchomień, które rozpoczynają się po zapisaniu zmian.

Nie ma opcji "użyj któregoś modelu, który jest tańszy". Wybór jest jawny dla każdego agenta.