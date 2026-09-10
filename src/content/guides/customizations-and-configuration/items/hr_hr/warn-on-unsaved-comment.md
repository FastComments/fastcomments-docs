[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, ako korisnik napiše komentar i zatim osvježi stranicu, zatvori karticu ili ode negdje drugdje prije slanja, skica se tiho izgubi.

Postavljanje **warnOnUnsavedComment** na true uzrokuje da preglednik pita korisnika da potvrdi napuštanje stranice dok bilo koje polje za komentar ili uređivanje u tijeku još uvijek sadrži tekst. Nakon što se komentar pošalje, tekst se očisti, pa se ne prikazuje nikakav upit.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Upozori na nespremljeni komentar'; code-example-end]

Upit koristi vlastiti dijalog preglednika. Moderni preglednici prikazuju vlastiti tekst i ignoriraju prilagođeni tekst, pa se poruka ne može prilagoditi.

Ova opcija učitava mali dodatak po potrebi, pa ne dodaje ništa widgetu za stranice koje je ne omoguće.