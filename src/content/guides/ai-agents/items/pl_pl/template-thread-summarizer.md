---
**ID szablonu:** `thread_summarizer`

Podsumowujący wątek publikuje neutralne, jednoakapitowe podsumowanie na końcu długiego wątku. Używa 30-minutowego opóźnienia, aby wątek mógł się uspokoić, zanim agent go przejrzy.

Wbudowane polecenie instruuje agenta, aby nie dodawał subiektywnego komentarza — to jest kluczowe. Bez niego model ma skłonność do formułowania się w stylu "moim zdaniem", co źle wygląda pod nazwą wyświetlaną konta.

### Wyzwalacze

- **Nowy komentarz dodany** (`COMMENT_ADD`).
- **Opóźnienie wyzwalacza**: 30 minut (1800 sekund). Zobacz [Wyzwalacze z opóźnieniem](#trigger-deferred-delay).

Opóźnienie 30 minut oznacza, że agent uruchamia się raz, pół godziny po pojawieniu się komentarza, działając na podstawie stanu wątku w tym momencie. To nie jest "podsumowuj przy każdej odpowiedzi" — kolejka wyzwalaczy z opóźnieniem łączy wiele zdarzeń nowego komentarza w tym samym wątku, ale nie usuwa duplikatów pomiędzy oddzielnymi oknami. Prawdopodobnie będziesz chciał **dodać niestandardową regułę w swoim promptcie** typu "nie publikuj nowego podsumowania, jeśli agent już podsumował ten wątek w ciągu ostatnich 24 godzin" i polegać na kontekście oraz [narzędziach pamięci](#tools-overview) agenta, aby to egzekwować.

### Dozwolone narzędzia

- [`write_comment`](#tools-overview) - publikuje samo podsumowanie.
- [`pin_comment`](#tools-overview) - przypina podsumowanie, aby czytelnicy widzieli je na górze wątku.
- [`unpin_comment`](#tools-overview) - odpina wcześniejsze podsumowanie tego samego agenta przed przypięciem nowego.

Podsumowujący nie może moderować ani wchodzić w interakcję z użytkownikami.

### Przypinanie podsumowania

Agent publikuje nowy komentarz za pomocą `write_comment`, a następnie wywołuje `pin_comment` z zwróconym ID komentarza. Przy kolejnych uruchomieniach dla tego samego wątku polecenie instruuje go, aby najpierw wywołał `unpin_comment` na swoim wcześniejszym podsumowaniu — sama platforma **nie** narzuca reguły jednego przypiętego komentarza na wątek, więc pozostawienie poprzedniego podsumowania przypiętego spowoduje obok siebie dwa przypięte podsumowania. Zaznacz "Dołącz komentarz nadrzędny i wcześniejsze odpowiedzi w tym samym wątku" w [Opcjach kontekstu](#context-options), aby agent mógł zobaczyć wcześniejsze przypięte podsumowanie.

### Zalecane uzupełnienia przed uruchomieniem

- **Zaznacz "Dołącz komentarz nadrzędny i wcześniejsze odpowiedzi w tym samym wątku"** w [Opcjach kontekstu](#context-options). Podsumowujący bez kontekstu wątku jest bezużyteczny.
- **Dostosuj regułę minimalnego rozmiaru wątku.** "Mniej niż 5 komentarzy" to domyślne ustawienie promptu, ale w zatłoczonych społecznościach 10–20 jest bardziej odpowiednie. Edytuj prompt bezpośrednio.
- **Ogranicz do określonych wzorców URL** jeśli chcesz podsumowania tylko na stronach długich, a nie na ogłoszeniach lub stronach produktów. Zobacz [Zakres: Filtry adresów URL i lokalizacji](#scope-url-locale).
- **Kontroluj koszty.** Podsumowywanie to szablon zużywający najwięcej tokenów, ponieważ czyta cały wątek przy każdym uruchomieniu. Ustaw ścisły [dzienny budżet](#budgets-overview) przed przełączeniem na Włączone.

### Unikanie powtarzających się podsumowań

Agent ma dostęp do [`save_memory`](#tools-overview) oraz [`search_memory`](#tools-overview) - możesz rozszerzyć prompt, aby nakazać mu zapisywanie notatek "podsumowano {thread urlId}" i sprawdzanie ich przed ponownym opublikowaniem. Pamięć jest współdzielona ze wszystkimi agentami w Twoim tenancie.

---