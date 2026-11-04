Stroški agenta temeljijo na **žetonih (tokens)**. Vsak klic LLM vrne število žetonov, platforma to pretvori v ameriške cente z uporabo cene modela na žeton, centi pa se obračunajo proti proračunom agenta in najemnika (tenant).

### Kaj se obračunava

- **Vsi klici LLM**, vključno s klicem, ki ne proizvede nobenega dejanja orodja ("agent se je odločil, da ne naredi ničesar"). Sklepanje (inference) se plača tudi, ko ne pride do nobenega dejanja.
- **Klici v suhem načinu (dry-run)**. Suhem način pomeni "ne ukrepaj, ampak vseeno pokliči LLM" - klic LLM stane enako. Glejte [Dry-Run Mode](#dry-run-mode).
- **Klici replay**. Replays so suhi zagoni nad zgodovinskimi komentarji. Stanejo žetone. Glejte [Test Runs (Replays)](#test-runs-replays).

### Kaj se ne obračunava

- **Sprožilci (triggers), ki nikoli ne proizvedejo klica LLM.** Primeri, opuščeni pred LLM (prekoračitev proračuna, omejitev hitrosti, neujemanje obsega, neveljavna obračunavanja, preprečevanje zank), stanejo nič žetonov. Glejte [Drop Reasons](#drop-reasons).
- **Odpošiljanje orodij.** Klic `pin_comment` ali katerega koli drugega orodja sam po sebi ne stane žetonov - le povratni klic LLM stane.
- **`search_memory`.** Ta je samo za branje in ne proizvede lastnega povratnega klica LLM.

### Stroški na zagon

En sam zagon agenta lahko večkrat pokliče LLM - rezultat vsakega klica orodja se vrne v model, da ta lahko pokliče drugo orodje ali zaključi. Torej je `tokensUsed` na zagonu vsota vseh povratnih klicev LLM v tem zagonu.

Največji prispevki k stroškom žetonov na zagon:

- **Dolgi [začetni pozivi](#personality-prompt) in [smernice skupnosti](#community-guidelines)** - gredo v vsak zagon.
- **[Možnosti konteksta](#context-options)** - kontekst niti, zgodovina uporabnika, metapodatki strani. Vsak dodaja žetone.
- **Samo besedilo komentarja** - dolgi komentarji stanejo več.
- **Več klicev orodij v enem zagonu** - sporočilo z rezultatom vsakega orodja se pošlje nazaj modelu.
- **Branje pomnilnika** - `search_memory` vrne do 25 zapisov (omejeno na 8000 znakov skupne vsebine). Večina teh bajtov gre v naslednji poziv.

**Max Tokens Per Trigger** (privzeto 20.000) omejuje velikost **odgovora** na klic LLM. Ne omejuje velikosti vnosa.

### Pretvorba žetonov v cente

Platforma uporablja enotno ceno na paket najemnika (`flexLLMCostCents` na `flexLLMUnit` žetonov). Cena na žeton je na ravni paketa, ne na ravni modela - oba razpoložljiva modela ([GLM 5.1 in GPT-OSS Turbo](#choosing-a-model)) se obračunata po isti ceni na danem paketu. [Run Detail View](#run-detail-view) prikaže strošek na zagon v vaši valuti, ko se zagon zaključi.

### Kje se stroški beležijo

Vsak zagon beleži svoje surovo število žetonov in strošek na zagon. Dnevne in mesečne vsote se zbirajo na [strani Analytics](#analytics-page).

### Kako brati stroške

- **Strošek na zagon**: [Run Detail View](#run-detail-view) -> polje `Cost`.
- **Dnevna / mesečna agregacija**: [stran Analytics](#analytics-page) -> grafikonja porabe proračuna in dnevnih stroškov.
- **Strošek na dejanje**: prav tako na Run Detail View, uporabno za nastavljanje, ko je zanka orodij agenta nenavadno dolga.

### Glejte tudi

- [Choosing a Model](#choosing-a-model) - večji vzvod na stroške.
- [Context Options](#context-options) - od koder prihajajo dodatni stroški.
- [Budgets Overview](#budgets-overview) - trde omejitve, ki preprečujejo nekontrolirane stroške.