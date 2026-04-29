Svaki agent koristi jedan od dva LLM modela, odabran u odjeljku **Model** obrasca za uređivanje.

### Dvije opcije

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - zadani. Viša kvaliteta zaključivanja, nešto sporiji po pozivu. Preporučuje se za agente u stilu moderiranja (`Moderator` template, bilo što što poziva `ban_user` ili `mark_comment_spam`) gdje je trošak pogrešne akcije visok.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - brži po pozivu, niža latencija. Preporučuje se za agente s velikim volumenom i niskim ulozima (pozdravljač, onaj koji pričvršćuje teme) gdje želite odgovore u roku od sekundi, a posljedice pogrešne akcije su zanemarive.

Oba modela podržavaju pozivanje funkcija, oba rade putem istog OpenAI-kompatibilnog API-ja, i oba dijele iste sheme po alatu - pa možete prebaciti spremljenog agenta između njih u bilo kojem trenutku bez dodatnih promjena konfiguracije.

### Razlike u troškovima

Dva modela imaju različite troškove po tokenu. Agenta [ograničenja budžeta](#budgets-overview) su izražena u valuti vašeg računa, ne u tokenima, pa prelazak s jednog modela na drugi mijenja koliko se pokretanja uklapa u vaše dnevne i mjesečne limite. Stranica [Povijest pokretanja](#run-history) prikazuje trošak po pokretanju u vašoj valuti nakon što se pokretanje dovrši - promatranje nekoliko pokretanja nakon promjene najjednostavniji je način da procijenite novu stopu trošenja.

### Tokeni po izvođenju

Upotreba tokena za odgovor modela bilježi se pri svakom okidaču kao **tokensUsed**. Uključeno je u webhook payloadove `trigger.succeeded` i `trigger.failed` (vidi [Podaci webhooka](#webhook-payloads)) i prikazuje se u [Prikaz detalja izvođenja](#run-detail-view). Količina ovisi o:

- Koliko [konteksta](#context-options) uključite - kontekst teme, povijest korisnika i metapodaci stranice svi dodaju tokene.
- Koliko su dugi vaš [početni prompt](#personality-prompt) i [smjernice zajednice](#community-guidelines).
- Koliko alata agent pozove u jednom izvođenju (svaki poziv alata i njegov rezultat putuje kroz model u oba smjera).

**Maksimalni tokeni po okidaču** (zadano 20,000) je gornja granica po izvođenju, postavljena po agentu.

### Promjena modela

Model možete promijeniti u obrascu za uređivanje u bilo kojem trenutku. Postojeća povijest pokretanja i analitika zadržavaju svoje izvorne podatke o tokenima i troškovima - ti se podaci bilježe pri izvođenju. Novi model primjenjuje se samo na pokretanja koja započnu nakon što spremite.

Ne postoji opcija "koristi koji god model je jeftiniji". Izbor se postavlja izričito za svakog agenta.