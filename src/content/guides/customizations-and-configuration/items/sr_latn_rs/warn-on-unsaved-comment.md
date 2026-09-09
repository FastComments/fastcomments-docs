---
[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Podrazumevano, ako korisnik napiše komentar, a zatim osveži stranicu, zatvori tab ili ode negde drugde pre slanja, skica se tiho gubi.

Postavljanje **warnOnUnsavedComment** na true čini da pregledač pita korisnika da potvrdi pre napuštanja stranice dok bilo koje polje za komentar ili uređivanje u toku još uvek sadrži tekst. Kada se komentar pošalje, tekst se briše, pa se ne prikazuje nijedan prompt.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Warn On Unsaved Comment'; code-example-end]

Prompt koristi sopstveni dijalog pregledača. Moderni pregledači prikazuju sopstveni tekst i ignorišu prilagođeni tekst, tako da se poruka ne može prilagoditi.

Ova opcija učitava mali dodatak po potrebi, tako da ne dodaje ništa widgetu za sajtove koji je ne omoguće.

---