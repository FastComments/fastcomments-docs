### Regenerisanje klijenta

Da biste regenerisali API klijenta iz najnovije OpenAPI specifikacije:

1. Uverite se da je FastComments server pokrenut lokalno na `http://localhost:3001`
2. Pokrenite skriptu za ažuriranje:

```bash
./update.sh
```

Ovo će:
- Preuzeti najnoviju OpenAPI specifikaciju
- Generisati Swift klijentski kod (sa API dokumentacijom u client/docs)
- Sastaviti paket kako biste proverili da sve radi