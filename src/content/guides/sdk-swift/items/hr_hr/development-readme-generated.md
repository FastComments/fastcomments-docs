### Regeneracija klijenta

Za regeneriranje API klijenta iz najnovije OpenAPI specifikacije:

1. Provjerite da je FastComments server pokrenut lokalno na `http://localhost:3001`
2. Pokrenite skriptu za ažuriranje:

```bash
./update.sh
```

Ovo će:
- Preuzeti najnoviju OpenAPI specifikaciju
- Generirati Swift klijentski kod (s dokumentacijom API-ja u client/docs)
- Izgraditi paket kako biste provjerili da sve radi