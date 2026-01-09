[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Domyślnie w FastComments komentowanie na żywo jest włączone.

Oznacza to, że każdy widz wątku komentarzy powinien widzieć tę samą zawartość.

Na przykład, jeśli komentarz zostanie dodany, ten komentarz powinien się pojawić. Jeśli komentarz zostanie edytowany lub usunięty,
to komentarze te zostaną edytowane lub usunięte dla wszystkich widzów wątku. To samo dotyczy głosów i wszystkich działań moderacyjnych.

Jednakże możemy to wyłączyć:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

To można też zrobić bez użycia kodu. Na stronie dostosowywania widżetu zobacz sekcję "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---