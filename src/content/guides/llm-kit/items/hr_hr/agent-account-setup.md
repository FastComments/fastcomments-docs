Agent za kodiranje poput Claude Code, Cursor ili MCP-baziranog asistenta može postaviti FastComments za vas bez da vi ispunjavate obrazac za prijavu. Ovo je korisno kada od agenta zatražite da "dodaje komentare mojoj stranici", a još nemate račun.

### Kako to funkcionira

1. Agent stvara novi račun i prima API ključ te poveznicu za preuzimanje (claim link). API ključ odmah radi, tako da agent može konfigurirati račun i instalirati widget na vašu stranicu.
2. Agent vam daje poveznicu za preuzimanje. Otvorite je u svom pregledniku, prijavite se ili stvorite prijavu i potvrdite preuzimanje. Račun je tada vaš: njime upravljate, kao i njegovom naplatom i API ključevima, iz nadzorne ploče. Stranica navodi API ključ koji agent drži, tako da ga možete opozvati ako više ne želite da agent, ili tko god ga vodi, ima pristup.
3. Ako nitko ne otvori poveznicu za preuzimanje u roku od 72 sata, račun i njegov ključ brišu se. Zatražite od agenta da stvori novi.

Dok se ne preuzme, račun ima ista ograničenja kao normalno besplatno probno razdoblje.

### Ako već imate račun

Svaka prijava posjeduje jedan račun. Ako otvorite poveznicu za preuzimanje dok ste prijavljeni u postojeći račun, stranica vam omogućuje izbor:

- **Pripoji mom računu** čini novi račun upravljanim tenantom računa u koji ste prijavljeni. To zahtijeva plaćeni plan s white labeling opcijom, a korištenje novog tenanta naplaćuje se vašem računu.
- **Odjavi se i preuzmi s drugom prijavom** vas odjavljuje i vraća na stranicu za preuzimanje kako biste ga preuzeli s drugom prijavom.

### Za autore agenata

Upute za agenta na [fastcomments.com/agents.md](https://fastcomments.com/agents.md) opisuju poziv za stvaranje računa, polja u odgovoru i kako predati poveznicu za preuzimanje osobi za koju radite. Poziv ne zahtijeva API ključ i ograničen je po brzini po IP adresi.