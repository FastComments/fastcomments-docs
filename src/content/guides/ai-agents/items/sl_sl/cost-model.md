Agent stroški so **na osnovi tokenov**. Vsak klic LLM vrne število tokenov; platforma to pretvori v ameriške cente (USD) z uporabo modelove cene na token, in ti centi se zaračunajo proti proračunom agenta in najemnika.

### Kaj se zaračunava

- **Vsi klici LLM**, vključno s klicem, ki ne povzroči nobenega ukrepa orodja ("agent se je odločil, da ne bo ničesar storil"). Za inferenco se plača tudi, kadar ne pride do nobenega ukrepa.
- **Klici suhega zagona**. Suhi zagon pomeni "ne izvajaj dejanj, vendar vseeno pokliči LLM" - klic LLM stane enako. Oglejte si [Način suhega zagona](#dry-run-mode).
- **Ponovitveni klici**. Ponovitve so suhi zagon proti zgodovinskim komentarjem. Stanejo tokene. Oglejte si [Testni zagoni (ponovitve)](#test-runs-replays).

### Kaj se ne zaračunava

- **Sprožilci, ki nikoli ne sprožijo klica LLM.** Primeri, kjer je postopek opuščen pred klicem LLM (prekoračen proračun, omejitev hitrosti, neusklajenost obsega, neveljavno zaračunavanje, preprečevanje zank) ne stanejo nobenih tokenov. Oglejte si [Razloge za opustitev](#drop-reasons).
- **Pošiljanje ukaza orodju.** Klic `pin_comment` ali katerega koli drugega orodja samo po sebi ne stane tokenov - računa se le LLM tur in nazaj.
- **`search_memory`.** Je samo za branje in ne sproži lastnega LLM klica.

### Stroški na zagon

En zagon agenta lahko pokliče LLM večkrat - rezultat vsakega klica orodja se vrne v model, da lahko pokliče drugo orodje ali konča. Zato je `tokensUsed` na zagon vsota vseh LLM tur v tem zagonu.

Največji dejavniki, ki prispevajo k token strošku na zagon:

- **Dolgi [začetni pozivi](#personality-prompt) in [smernice skupnosti](#community-guidelines)** - vključeni so v vsak zagon.
- **[Možnosti konteksta](#context-options)** - kontekst teme, zgodovina uporabnika, metapodatki strani. Vsak poveča število tokenov.
- **Samo besedilo komentarja** - dolgi komentarji stanejo več.
- **Več klicev orodij v enem zagonu** - vsako sporočilo z rezultatom orodja se pošlje nazaj modelu.
- **Branje iz pomnilnika** - `search_memory` vrne do 25 zapisov (omejeno na 8000 znakov skupne vsebine). Večina teh bajtov gre v naslednji poziv.

**Največje število tokenov na sprožilec** (privzeto 20.000) omejuje velikost **odgovora** na klic LLM. Ne omejuje velikosti vnosa.

### Pretvorba tokenov v cente

Platforma uporablja eno stopnjo na paket najemnika (`flexLLMCostCents` na `flexLLMUnit` tokenov). Strošek na token je na ravni paketa, ne na ravni modela - oba razpoložljiva modela ([GLM 5.1 in GPT-OSS Turbo](#choosing-a-model)) se zaračunata po enaki stopnji v določenem paketu. [Pogled podrobnosti zagona](#run-detail-view) prikaže strošek na zagon v vaši valuti, ko se zagon zaključi.

### Kje so zabeleženi stroški

Vsak zagon zabeleži surovo število tokenov in strošek na zagon. Dnevne in mesečne vsote se seštejejo na [Stran analitike](#analytics-page).

### Kako razumeti stroške

- **Strošek na zagon**: [Pogled podrobnosti zagona](#run-detail-view) -> polje `Cost`.
- **Dnevni / mesečni agregat**: [Stran analitike](#analytics-page) -> grafikoni porabe proračuna in dnevnih stroškov.
- **Strošek na dejanje**: prav tako v [Pogledu podrobnosti zagona](#run-detail-view), uporaben za prilagajanje, ko je zanka orodij agenta nenavadno dolga.

### Oglejte si tudi

- [Izbira modela](#choosing-a-model) - največji vpliv na stroške.
- [Možnosti konteksta](#context-options) - od kod prihajajo dodatni stroški.
- [Pregled proračunov](#budgets-overview) - trde omejitve, ki preprečujejo nenadzorovane stroške.