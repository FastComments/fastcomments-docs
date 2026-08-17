Run History je dnevnik po agentu za vsako sprožitev, ki se je izvedla. Dostopen je s strani seznama agentov prek gumba **Runs**, ali neposredno na `/auth/my-account/ai-agents/{agentId}/runs`.

### Kaj je na strani

Stran vsebuje paginirano tabelo z eno vrstico na izvajanje:

| Stolpec | Pomen |
|---|---|
| Datum | Kdaj je bila sprožitev aktivirana (ali kdaj je bila odložena sprožitev izvedena). |
| Stanje | **Started**, **Success**, ali **Error**. Oznaka **Dry Run** je prikazana poleg, če je bilo izvajanje v načinu suhega zagona. |
| Strošek | Strošek na izvajanje v valuti vašega najemnika. Prazno za izvajanja v teku (Started). |
| Dejanja | Število klicev orodij v tem izvajanju. |
| Podrobnosti | Gumb **View**, ki odpre [Run Detail View](#run-detail-view). |

### Pomeni stanja

- **Started** – izvajanje je v teku ali je prekinjeno pred dokončanjem. Izvajanje, ki ostane v stanju "Started" nenavadno dolgo, običajno pomeni časovno omejitev klica LLM.  
- **Error** – izvajanje se je zaključilo, vendar je nekje spodletelo – klic LLM je vrnil napako, pošiljanje orodja je spodletelo ipd. Podrobni pogled vsebuje specifično napako.  
- **Success** – izvajanje se je zaključilo brez napake. Agent je lahko izvedel nič, eno ali več dejanj.

### Prazno stanje

Ko agent nima izvajanj, stran prikaže: "Za tega agenta še ni izvajanj. Omogočena izvajanja se bodo pojavila tukaj, ko se sproži sprožitev; uporabite Test run, da si ogledate, kaj bi ta agent storil pri preteklih komentarjih."

Ta zadnji del je namenjen – [test run flow](#test-runs-replays) je priporočljiv način za napolnitev Zgodovine izvajanj pri novem agentu.

### Kaj ni na strani zgodovine izvajanj

- **Live triggers that never dispatched** – sprožitev, ki je bila opuščena zaradi proračuna, obsega ali omejitve hitrosti, se na tej strani ne prikaže. Pojavijo se na [Analytics page](#analytics-page) pod "Triggers skipped".  
- **Approvals** – čakajoča odobritve za dejanja, izvedena v tem izvajanju, so v [approvals inbox](#approval-workflow). Dejanje se prikaže v podrobnem pogledu izvajanja kot **Pending approval**.

### Zadrževanje

Posamezni zapisi izvajanj se hranijo 90 dni, po tem pa se izvajanje odstrani iz zgodovine. Stroški in število sprožitev se še naprej seštevajo v dolgoročnih povzetkih analitike, zato [Analytics page](#analytics-page) še vedno prikazuje zgodovinske skupne vrednosti po tem obdobju.

### Ponovitve

Izvajanja, ustvarjena s ponovitvijo, so privzeto izključena iz pogleda live-runs. Stran [Test Runs (Replays)](#test-runs-replays) je mesto, kjer jih lahko vidite.

### Filtriranje med agenti

Tabela izvajanj je po agentih. Ni pogleda izvajanj med agenti – [Analytics page](#analytics-page) je povzetek med agenti. Če morate pregledati izvajanja med več agenti, so dogodki [Webhooks](#webhooks-overview) `trigger.succeeded` in `trigger.failed`, ki jih pošljete v svoj sistem.