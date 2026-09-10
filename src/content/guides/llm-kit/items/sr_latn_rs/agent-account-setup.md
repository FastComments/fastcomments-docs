---
A coding agent, poput Claude Code, Cursor ili asistenta zasnovanog na MCP, može postaviti FastComments za vas bez da popunjavate formular za registraciju. Ovo je korisno kada zatražite od agenta da "dodaje komentare na moj sajt" i još nemate nalog.

### Kako funkcioniše

1. Agent kreira novi nalog i dobija API ključ i claim link. API ključ odmah funkcioniše, tako da agent može konfigurisati nalog i instalirati widget na vaš sajt.  
2. Agent vam daje claim link. Otvorite ga u vašem pretraživaču, prijavite se ili kreirajte login, i potvrdite claim. Nalog tada postaje vaš: vi upravljate njime, njegovim naplatama i API ključevima putem kontrolne table. Stranica prikazuje API ključ koji agent drži, kako biste ga mogli opozvati ako više ne želite da agent, ili ko ga koristi, ima pristup.  
3. Ako niko ne otvori claim link u roku od 72 sata, nalog i njegov ključ se brišu. Zamolite agenta da kreira novi.

Dok se ne preuzme, nalog ima ista ograničenja kao obična besplatna probna verzija.

### Ako već imate nalog

Svaka prijava (login) poseduje jedan nalog. Ako otvorite claim link dok ste prijavljeni na postojeći nalog, stranica vam omogućava da izaberete:

- **Attach to my account** čini novi nalog upravljanim tenantom onog na koji ste prijavljeni. Ovo zahteva plaćeni plan sa white labeling-om, a korišćenje novog tenant-a se naplaćuje vašem nalogu.  
- **Sign out and claim with another login** odjavljuje vas i vraća na stranicu za claim kako biste je mogli preuzeti drugim login-om.

### Za autore agenata

Uputstva za agente na [fastcomments.com/agents.md](https://fastcomments.com/agents.md) opisuju poziv za kreiranje naloga, polja u odgovoru i kako da predate claim link osobi za koju radite. Poziv ne zahteva API ključ i ograničen je po brzini po IP adresi.

---