Vsak agent teče na enem od dveh LLM modelov, izbranih v razdelku **Model** na obrazcu za urejanje.

### Dve možnosti

- **GLM 5.1 (DeepInfra) - Pametnejši, nekoliko počasnejši** - privzeto. Višja kakovost sklepanja, nekoliko počasnejši pri posameznem klicu. Priporočeno za agente za moderiranje (`Moderator` template, karkoli, kar kliče `ban_user` ali `mark_comment_spam`), kjer je cena napačnega klica visoka.

- **GPT-OSS 120B Turbo (DeepInfra) - Hitrejši** - hitrejši pri posameznem klicu, nižja zakasnitev. Priporočeno za agente z velikim obsegom in nizko pomembnostjo (pozdravitelj, pripenjač teme), kjer želite odgovore v nekaj sekundah in so posledice napačnega klica manjše.

Oba modela podpirata klic funkcij, oba tečeta prek iste OpenAI-kompatibilne API, in oba delita enake sheme na orodje - zato lahko kadarkoli preklopite shranjenega agenta med njima brez dodatnih sprememb konfiguracije.

### Razlike v stroških

Oba modela imata različne stroške na token. Agentove [omejitve proračuna](#budgets-overview) so izražene v valuti vašega računa, ne v tokenih, zato preklop z enega modela na drugega spremeni, koliko zagonov se prilega v vaše dnevne in mesečne omejitve. Stran [Zgodovina zagonov](#run-history) prikaže strošek na posamezen zagon v vaši valuti, ko se zagon zaključi - opazovanje nekaj zagonov po preklopu je najlažji način za oceno nove stopnje porabe.

### Tokeni na zagon

Poraba tokenov za odgovor modela se beleži pri vsakem sprožilcu kot **tokensUsed**. Vključena je v webhook payloadih `trigger.succeeded` in `trigger.failed` (glej [Webhook payloadi](#webhook-payloads)) in je prikazana v [Pogled podrobnosti izvajanja](#run-detail-view). Količina je odvisna od:

- Koliko [Kontekst](#context-options) vključite - kontekst niti, zgodovina uporabnika in metapodatki strani vsi povečajo porabo tokenov.
- Kako dolga sta vaš [Začetni poziv](#personality-prompt) in [Smernice skupnosti](#community-guidelines).
- Koliko orodij agent pokliče v enem zagonu (vsak klic orodja in njegov rezultat opravi krožno pot skozi model).

**Max Tokens Per Trigger** (privzeto 20,000) je zgornja meja na zagon, nastavljena za posameznega agenta.

### Preklop modelov

Model lahko preklopite na obrazcu za urejanje kadarkoli. Obstoječa zgodovina zagonov in analitika ohranita svoje prvotne številke tokenov in stroškov - te so zabeležene ob izvajanju. Novi model se uporablja le za zagone, ki se začnejo po tem, ko shranite.

Ni možnosti "uporabi kateri koli model, ki je cenejši". Izbira je izrecna za posameznega agenta.