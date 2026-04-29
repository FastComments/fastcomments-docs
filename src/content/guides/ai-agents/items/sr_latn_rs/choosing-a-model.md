Svaki agent izvršava se protiv jednog od dva LLM modela, biranih u **Model** sekciji obrasca za izmenu.

### Dve opcije

- **GLM 5.1 (DeepInfra) - Pametniji, malo sporiji** - podrazumevani. Viši kvalitet rezonovanja, nešto sporiji po pozivu. Preporučuje se za agente u stilu moderacije (`Moderator` template, sve što poziva `ban_user` ili `mark_comment_spam`) gde je cena pogrešnog poziva visoka.

- **GPT-OSS 120B Turbo (DeepInfra) - Brži** - brži po pozivu, niža latencija. Preporučuje se za agente sa velikim obimom i niskim ulozima (pozdravljač, fiksirač teme) gde želite odgovore u roku od par sekundi i posledice pogrešnog poziva su manje važne.

Oba modela podržavaju pozivanje funkcija, oba rade preko iste OpenAI-kompatibilne API, i oba dele iste šeme po alatu - pa možete prebaciti sačuvanog agenta između njih u bilo kom trenutku bez dodatnih konfiguracionih promena.

### Razlike u troškovima

Dva modela imaju različite cene po tokenu. Agentova [ograničenja budžeta](#budgets-overview) su iskazana u valuti vašeg naloga, a ne u tokenima, tako da prelazak sa jednog modela na drugi menja koliko izvršavanja stane u vaše dnevne i mesečne limite. Stranica [Run History](#run-history) prikazuje cenu po izvršenju u vašoj valuti kada se izvršavanje završi - posmatranje nekoliko izvršavanja nakon promene je najlakši način da procenite novu stopu trošenja.

### Tokeni po izvršenju

Korišćenje tokena u odgovoru modela beleži se na svakom okidaču kao **tokensUsed**. To je uključeno u payload-ove webhook-ova `trigger.succeeded` i `trigger.failed` (vidi [Webhook Payloads](#webhook-payloads)) i prikazano u [Run Detail View](#run-detail-view). Količina zavisi od:

- Koliko [Context](#context-options) uključite - kontekst teme, istorija korisnika i metapodaci stranice svi dodaju tokene.
- Koliko su dugi vaš [Initial prompt](#personality-prompt) i [Community guidelines](#community-guidelines).
- Koliko alata agent pozove u jednom izvršenju (svaki poziv alata i njegov rezultat kružno prolaze kroz model).

**Maksimalan broj tokena po okidaču** (podrazumevano 20.000) je gornja granica po izvršenju, podešava se po agentu.

### Promena modela

Model možete promeniti u formi za izmenu u bilo kom trenutku. Postojeća istorija pokretanja i analitika zadržavaju svoje originalne vrednosti tokena i troškova - one se beleže u vreme izvršenja. Novi model se primenjuje samo na pokretanja koja započnu nakon što sačuvate.

Ne postoji opcija "koristi bilo koji model koji je jeftiniji". Izbor je eksplicitan po agentu.

---