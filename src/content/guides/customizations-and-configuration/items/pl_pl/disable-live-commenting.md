[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments ma włączone komentarze na żywo.

Oznacza to, że każdy oglądający wątek komentarzy powinien widzieć tę samą treść.

Na przykład, jeśli zostanie dodany komentarz, powinien się on wyświetlić. Jeśli komentarz zostanie edytowany lub usunięty,
to te komentarze będą edytowane lub usuwane dla wszystkich oglądających wątek. To samo dotyczy głosów i wszystkich działań moderacyjnych.

Jednak możemy to wyłączyć:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Można to również zrobić bez kodu. Na stronie dostosowywania widgetu, zobacz sekcję „Disable Live Commenting”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Sekcja wyłączania komentarzy na żywo na stronie dostosowywania widgetu, wyłączająca aktualizacje wątku w czasie rzeczywistym'; title='Wyłącz komentarze na żywo' app-screenshot-end]