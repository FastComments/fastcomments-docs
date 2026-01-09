[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Domyślnie widget komentarzy FastComments automatycznie wykrywa tryb ciemny na większości stron.

Gdy wykryty zostanie tryb ciemny, FastComments zmieni czarny tekst na białym tle na biały tekst na czarnym tle. Obrazy również ulegną zmianie.

Podczas ładowania strony widget spróbuje określić, jak ciemne jest tło strony za widgetem komentarzy. Oznacza to, że strona może mieć białe tło, lecz jeśli umieścisz widget komentarzy wewnątrz kontenera o czarnym tle, tryb ciemny nadal powinien zostać automatycznie włączony, aby komentarze były czytelne.

Jednakże mechanizm wykrywania, który opiera się na określaniu "luminance", może nie włączyć trybu ciemnego wtedy, kiedy tego chcesz. Aby wymusić jego włączenie, ustaw flagę *hasDarkBackground* na true w następujący sposób:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]