Udløses, når en moderator tildeler et badge til en bruger.

### Kontekst, som agenten modtager

- Det **badge-ID**, der er tildelt.
- **ID for den udløsende bruger** - moderatoren, der tildelte badget.
- Valgfri tråd-, brugerhistorik- eller sidekontekst som konfigureret.

Udløsningsstedet indeholder **ikke** en `commentId` i trigger-payloaden, selvom badget blev tildelt med henblik på en bestemt kommentar.

### Hvem udløser dette

En handling udført af en menneskelig moderator.

### Bemærkninger

- Kun badge-ID'et er inkluderet; agenten modtager ikke badge-metadata (navn, billede). Hvis agenten har brug for at afgøre *hvilket* badge der blev tildelt, indlejre den kontekst i [indledende prompt](#personality-prompt) eller [fællesskabsretningslinjer](#community-guidelines).
- Triggeren udløses én gang per badge-uddeling, ikke per bruger. At give samme badge til en bruger to gange udløser det to gange (hver uddeling er en separat begivenhed).

### Typiske anvendelser

- **Gensidig anerkendelse** - en agent kan poste et "tak for det gode bidrag"-svar, når et bestemt badge tildeles.
- **Ekstern anerkendelsesarbejdsgang** via [Webhooks](#webhooks-overview) - spejl badge-tildelinger ind i dit eget system for brugerengagement.
- **Hukommelsesregistrering** - noter som "denne bruger er en anerkendt bidragyder", som fremtidige moderationsagenter bør tillægge vægt i deres beslutninger.

---