[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Privzeto, če uporabnik napiše komentar in nato osveži stran, zapre zavihek ali zapusti stran, preden ga pošlje, se osnutek tiho izgubi.

Nastavitev **warnOnUnsavedComment** na true povzroči, da brskalnik uporabnika vpraša za potrditev, preden zapusti stran, medtem ko katerokoli polje za komentar ali urejanje, ki je v teku, še vsebuje besedilo. Ko je komentar poslan, se besedilo počisti, zato se poziv ne prikaže.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Opozori na neshranjene komentarje'; code-example-end]

Poziv uporablja lastni dialog brskalnika. Sodobni brskalniki prikazujejo svoje besedilo in ignorirajo prilagojeno besedilo, zato sporočila ni mogoče prilagoditi.

Ta možnost po potrebi naloži majhno razširitev, zato ne doda ničesar v gradnik za spletna mesta, ki je ne omogočijo.