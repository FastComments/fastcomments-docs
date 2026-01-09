[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Privzeto bo pripomoček za komentarje FastComments samodejno zaznal temni način na večini spletnih mest.

Ko je zaznan temni način, bo FastComments prešel iz črnega besedila na belem ozadju v belo besedilo na črnem ozadju. Spremembe bodo vidne tudi pri slikah.

Ob nalaganju strani bo pripomoček poskušal določiti, kako temno je ozadje strani za pripomočkom za komentarje. To pomeni, da
lahko stran ima belo ozadje, vendar če pripomoček za komentarje postavite v vsebnik s črnim ozadjem, bi moral biti temni način
še vedno samodejno omogočen, da bodo komentarji berljivi.

Vendar pa mehanizem zaznavanja, ki temelji na določanju "svetilnosti", morda ne bo omogočil temnega načina, ko ga želite. Če ga želite prisilno omogočiti, nastavite
zastavico *hasDarkBackground* na true kot sledi:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]