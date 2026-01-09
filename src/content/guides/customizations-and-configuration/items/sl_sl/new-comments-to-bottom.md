[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Privzeto se novi komentarji v živo pojavijo na vrhu seznama komentarjev, ko so objavljeni v realnem času.

Ko je ta možnost omogočena, se novi komentarji v živo namesto tega dodajo na dno seznama. To vpliva na način prikaza komentarjev, ko so objavljeni v živo, medtem ko uporabniki gledajo nit komentarjev.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Ko je ta nastavitev omogočena:
- Novi komentarji v živo, ki jih objavijo drugi uporabniki, se bodo pojavili na dnu seznama komentarjev
- Uporabniki bodo v realnem času videli, kako se novi komentarji pojavljajo pod obstoječimi komentarji
- To vpliva samo na posodobitve komentarjev v živo - ne na začetno nalaganje strani
- To lahko pomaga ohranjati tok branja, ko uporabniki spremljajo razpravo

Upoštevajte, da ta nastavitev vpliva le na to, kje so novi komentarji v živo umeščeni, ko prispevajo v realnem času. Ne vpliva na začetni vrstni red razvrščanja ob nalaganju strani.