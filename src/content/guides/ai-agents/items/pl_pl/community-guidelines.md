---
Pole **Community guidelines** w formularzu edycji to opcjonalny blok tekstu polityki dołączany do wiadomości kontekstowej roli użytkownika przy każdym uruchomieniu tego agenta. Jest on ogrodzony jako tekst nierozpoznawany jako zaufany (to samo ogrodzenie, które platforma stosuje do treści komentarzy i innych danych dostarczonych przez użytkowników), więc model traktuje go jako odniesienie do polityki, a nie jako instrukcje systemowe. To kanoniczne miejsce do zapisania „jakie zachowania są dozwolone, a jakie zabronione na tej stronie”, aby agent stosował je konsekwentnie.

### Jak różni się od początkowego promptu

- **Initial prompt** - rola agenta i styl podejmowania decyzji. „Jesteś moderatorem. Preferuj ostrzeżenia zamiast banów.”
- **Community guidelines** - zasady twojej społeczności, w języku polityki. „Brak ataków personalnych. Brak linków promocyjnych z kont młodszych niż 24 godziny. Off-topicowe komentarze mogą zostać usunięte, jeśli wątek jest rozgrzany.”

Oba trafiają do tego samego okna kontekstowego, ale wchodzą na różnych warstwach — initial prompt jest częścią roli systemowej, dokument wytycznych to tekst ogrodzony we wiadomości kontekstowej roli użytkownika. To rozdzielenie ułatwia edycję, gdy chcesz zaktualizować jedno bez ponownego czytania drugiego.

### Czym jest dobry dokument wytycznych

Krótki, konkretny dokument napisany przez człowieka. Listy działają lepiej niż proza:

[inline-code-attrs-start title = 'Przykład wytycznych społeczności'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

Agent stosuje to przy każdym uruchomieniu. Jeśli zmienisz wytyczne, zmiana obowiązuje przy następnym wyzwoleniu — wcześniejsze uruchomienia nie są poddawane retroaktywnej ponownej ocenie.

### Czego tu nie umieszczać

- **Output formatting instructions** („odpowiedz w HTML”, „użyj emoji”). Należą one do [initial prompt](#personality-prompt).
- **Localized text.** Dokument wytycznych, podobnie jak prompt, jest **tylko po angielsku** z tego samego powodu — maszynowe tłumaczenie może cicho zmienić zachowanie agenta. Jeśli masz polityki różniące się w zależności od lokalizacji, wpisz je wszystkie po angielsku w tym jednym dokumencie i ustrukturyzuj dokument jako „dla stron w języku niemieckim: ...”
- **Długie cytaty z zewnętrznych polityk.** Parafrazuj. Długi kontekst zwiększa koszt tokenów przy każdym uruchomieniu.
- **Dane osobowe lub sekrety.** Ten tekst jest wysyłany do dostawcy LLM przy każdym uruchomieniu.

### Długość

Pole jest ograniczone do **4000 znaków** (egzekwowane zarówno przez formularz, jak i trasę zapisu). Koszt tokenów przy każdym uruchomieniu jest proporcjonalny do długości, więc nawet w obrębie limitu kilkaset słów zazwyczaj wystarcza. Jeśli twoje zasady społeczności zajmują wiele stron, podsumuj części, które agent potrzebuje, w dokumencie specjalnie przeznaczonym do tego pola.

### Wersjonowanie

Nie ma wbudowanej historii wersji dla dokumentu wytycznych — agent używa najnowszej zapisanej wartości. Jeśli chcesz historię, skopiuj dokument do własnego systemu śledzenia przed każdą większą edycją. Przebieg [Refine Prompts](#refining-prompts) może zapisywać zmiany *initial prompt*, ale nie wersjonuje dokumentu wytycznych.

---