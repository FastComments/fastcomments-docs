**Poskusni zagon** je varnostni način, v katerem se vsak nov agent zažene. Agent izvaja celoten potek razen dela, kjer posega v vašo skupnost.

### Kaj se izvaja v poskusnem zagonu

- Sprožilci se sprožijo kot običajno.
- Agentov poziv, [smernice skupnosti](#community-guidelines) in [kontekst](#context-options) so sestavljeni.
- LLM se pokliče.
- Model izbere klice orodij in poda utemeljitve + ocene zaupanja.
- Zagon je zabeležen z značko **Poskusni zagon**, tako da je jasno ločen od izvedb v živo.

### Kaj se ne izvaja v poskusnem zagonu

- Noben komentar ni objavljen, noben glas ni oddan, noben komentar ni pripet/odpet/zaklenjen/odklenjen.
- Noben komentar ni označen kot spam, odobren ali pregledan.
- Noben uporabnik ni prepovedan, opozorjen ali mu ni podeljena značka.
- E-pošta ni poslana.
- Spomin se ne zapiše. (Da - vključno s spominom. Agenti v poskusnem zagonu ne morejo zastrupljati skupnega sklada spomina s hipotetičnimi odločitvami.)
- Za klice orodij se webhooks ne sprožijo. (Webhooki na nivoju sprožilca `trigger.succeeded` / `trigger.failed` se še vedno sprožijo in v payloadu vključujejo `wasDryRun: true`. Glej [Podatki webhooka](#webhook-payloads).)

### Kaj stane

Poskusni zagon izvede **isti klic LLM**, kot bi ga izvedel omogočen zagon. Žetoni se zaračunavajo, veljajo [omejitve proračuna](#budgets-overview) in izvedbe se štejejo proti dnevnim/mesečnim omejitvam na agenta in na najemnika.

Ta strošek je cena za zanesljiv predogled. Način "preskoči klic LLM" vam ne bi dal nobenega signala o tem, kako bi se agent obnašal.

### Branje rezultatov poskusnega zagona

V [Zgodovini izvedb](#run-history) so poskusi označeni z značko **Poskusni zagon** v stolpcu status. Dejanji znotraj vsakega zagona so videti identični dejanskim dejanjem - isto ime orodja, isti argumenti, ista utemeljitev in ocena zaupanja - razen tega, da nobeno od njih ni bilo dejansko izvedeno.

Stran z analitiko [Analytics page](#analytics-page) razčleni "poskusni zagon proti izvedbam v živo" po mesecih, tako da lahko vidite, koliko vaše porabe žetonov je šlo v opazovanje.

### Preklop iz poskusnega zagona

Uredite agenta in spremenite **Status** v **Omogočeno**. Naslednji sprožilec se izvede v živo.

Lahko tudi preklopite v drugo smer — iz **Omogočeno** nazaj v **Poskusni zagon** — če agent začne delati stvari, ki vam niso všeč. Ni kazni.

### Ponavljanja (replay) prisilijo poskusni zagon

Funkcija [Preizkusi (ponovitve)](#test-runs-replays) zažene agenta proti zgodovinskim komentarjem **vedno v poskusnem zagonu**, ne glede na shranjeni status agenta. Ponavljanja ne morejo izvesti dejanskih ukrepov na preteklih komentarjih. To je namenjeno — ponovitev je orodje za predogled, ne orodje za moderacijo.