Domyślnie FastComments tłumaczy własny interfejs widżetu (przyciski, etykiety, komunikaty o błędach) na język widza, ale komentarze są wyświetlane dokładnie tak, jak zostały napisane.

Tłumaczenie komentarzy dodaje tłumaczenie samych komentarzy, dzięki czemu społeczność pisząca w więcej niż jednym języku może czytać każdy komentarz w swoim języku.

Istnieją dwa tryby:

- **Automatyczny**: komentarze napisane w języku innym niż język widżetu są wyświetlane przetłumaczone. Linia „Przetłumaczono z hiszpańskiego” pojawia się pod komentarzem z linkiem **Pokaż oryginał**, który przełącza między tłumaczeniem a nieprzetłumaczonym tekstem.
- **Ręczny**: każdy komentarz otrzymuje link **Tłumacz**. Kliknięcie go tłumaczy dany komentarz na język widza, z tym samym przełącznikiem **Pokaż oryginał** po tłumaczeniu. Komentarze, które już są w języku widza, informują o tym zamiast tego.

Językiem docelowym jest język widżetu: ustawienie [Lokalizacja / Język](#locale) w tej samej regule dostosowywania, jeśli jest ustawione, w przeciwnym razie język przeglądarki odwiedzającego. Od odwiedzającego nie jest nic wymagane, nie potrzebuje konta, rozszerzenia ani dodatkowego oprogramowania, i działa tak samo w osadzonym widżecie oraz na samodzielnych stronach komentarzy.

To jest włączane per <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">reguła dostosowywania</a>, więc może mieć zastosowanie do całej witryny, jednej domeny lub pojedynczej strony. Domyślnie jest wyłączone.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Menu rozwijane Tłumaczenie komentarzy na stronie dostosowywania widżetu z opcjami Wyłączone, Automatyczne i Ręczne'; title='Tłumaczenie komentarzy' app-screenshot-end]

Tłumaczenie jest ustawieniem po stronie serwera. Przekazanie `commentTranslation` w konfiguracji JavaScript widżetu nie włącza go; robi to reguła.

Każdy komentarz jest tłumaczony co najwyżej raz na język docelowy i wynik jest przechowywany razem z komentarzem, więc komentarz odczytany przez tysiąc hiszpańskojęzycznych użytkowników jest tłumaczony raz. Jeśli autor lub moderator edytuje komentarz, przechowywane tłumaczenie jest odrzucane i komentarz jest tłumaczony ponownie przy następnym wyświetleniu. Przetłumaczony tekst przechodzi tę samą sanitację i zasady linkowania co oryginalny komentarz. Komentarze, które są zbyt długie lub których język nie może zostać wykryty, są wyświetlane nieprzetłumaczone.

Użycie jest rozliczane jako tokeny LLM w tej samej stawce co [Agenci AI](/guide-ai-agents.html#plans-and-eligibility) i pojawia się w tym samym miejscu w szczegółach rozliczeń.