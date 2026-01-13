[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Število komentarjev, prikazano na vrhu pripomočka za komentarje, lahko prikaže bodisi vse komentarje na najvišji ravni, to pomeni odgovore, ki so neposredni odgovori na stran ali članek, ali pa je lahko to število **vseh** gnezdenih komentarjev.

Privzeto je to `true` - to je števec slednjega - vseh komentarjev. V starejših različicah pripomočka za komentarje je bila privzeta vrednost `false`.

Obnašanje lahko spremenimo, tako da bo štel **vse** gnezdene komentarje, tako da zastavico **countAll** nastavite na true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Če želimo, da števec odraža le komentarje na najvišji ravni, zastavico nastavimo na false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

To trenutno ni mogoče prilagoditi brez sprememb kode.