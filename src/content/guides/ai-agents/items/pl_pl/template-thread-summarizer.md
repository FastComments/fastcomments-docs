---
**Template ID:** `thread_summarizer`

The Thread Summarizer publikuje neutralne, jednoakapitowe podsumowanie na końcu długiego wątku. Używa 30-minutowego opóźnienia, aby wątek mógł się ustabilizować zanim agent go przejrzy.

### Built-in initial prompt

[inline-code-attrs-start title = 'Wstępny prompt szablonu podsumowania wątku'; type='text' inline-code-attrs-end]
[inline-code-start]
Publikujesz neutralne podsumowania wątków. Nie podsumowuj wątków, które mają mniej niż 5 komentarzy. W przypadku dłuższych wątków podsumuj główne stanowiska, niezgodności i otwarte pytania w jednym krótkim akapicie. Nie zajmuj strony i nie formułuj opinii. Po opublikowaniu podsumowania przypnij je. Jeśli wcześniejsze podsumowanie autorstwa Ciebie jest już przypięte w tym wątku, odprzypnij je przed przypięciem nowego.
[inline-code-end]

Instrukcja „nie formułuj opinii” jest kluczowa. Bez niej model skłania się do sformułowań typu „moim zdaniem”, które źle wyglądają pod nazwą wyświetlaną Twojego konta.

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). See [Wyzwalacze z opóźnieniem](#trigger-deferred-delay).

30-minutowe opóźnienie oznacza, że agent uruchamia się raz, pół godziny po dodaniu komentarza, i działa na tym, jak wygląda wątek w tym momencie. To nie jest „podsumowuj przy każdej odpowiedzi” — kolejka wyzwalaczy z opóźnieniem scala wiele zdarzeń dodania komentarza w tym samym wątku, ale nie deduplikuje ich w oddzielnych oknach czasowych. Prawdopodobnie zechcesz **dodać regułę w swoim promptcie** typu „nie publikuj nowego podsumowania jeśli agent już podsumował ten wątek w ciągu ostatnich 24 godzin” i polegać na kontekście oraz [narzędziach pamięci](#tools-overview) agenta, aby to egzekwować.

### Allowed tools

- [`write_comment`](#tools-overview) - publikuje samo podsumowanie.
- [`pin_comment`](#tools-overview) - przypina podsumowanie, aby czytelnicy widzieli je na górze wątku.
- [`unpin_comment`](#tools-overview) - odprzypina wcześniejsze podsumowanie tego samego agenta przed przypięciem nowego.

Agent podsumowujący nie może moderować ani wchodzić w interakcję z użytkownikami.

### Pinning the summary

Agent publikuje nowy komentarz za pomocą `write_comment`, a następnie wywołuje `pin_comment` z zwróconym ID komentarza. Przy kolejnych uruchomieniach dla tego samego wątku prompt instruuje go, aby najpierw wywołał `unpin_comment` dla swojego wcześniejszego podsumowania — sama platforma **nie** wymusza zasady jednego przypiętego komentarza na wątek, więc pozostawienie poprzedniego podsumowania przypiętego skutkować będzie dwoma przypiętymi podsumowaniami obok siebie. Zaznacz "Include parent comment and prior replies in the same thread" w [Opcjach kontekstu](#context-options), aby agent mógł zobaczyć wcześniejsze przypięte podsumowanie.

### Recommended additions before going live

- **Zaznacz "Include parent comment and prior replies in the same thread"** w [Opcjach kontekstu](#context-options). Podsumowujący bez kontekstu wątku jest bezużyteczny.
- **Dopasuj regułę minimalnego rozmiaru wątku.** "Fewer than 5 comments" to domyślne ustawienie promptu, ale w ruchliwych społecznościach bardziej odpowiednie są wartości 10–20. Edytuj prompt bezpośrednio.
- **Ogranicz do określonych wzorców URL**, jeśli chcesz podsumowania tylko na stronach długiej formy, a nie na ogłoszeniach czy stronach produktowych. Zobacz [Zakres: filtry URL i lokalizacji](#scope-url-locale).
- **Kontroluj koszty.** Podsumowywanie zużywa najwięcej tokenów, ponieważ przy każdym uruchomieniu czyta cały wątek. Ustaw rygorystyczny [dzienny budżet](#budgets-overview) przed przełączeniem na Włączone.

### Avoiding repeat summaries

Agent ma dostęp do [`save_memory`](#tools-overview) i [`search_memory`](#tools-overview) - możesz rozszerzyć prompt, aby instruować go, by zapisywał notatki „podsumowano {thread urlId}” i sprawdzał je przed ponownym publikowaniem. Pamięć jest współdzielona przez wszystkich agentów w Twoim tenantcie.

---