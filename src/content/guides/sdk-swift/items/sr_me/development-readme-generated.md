### Ponovno generisanje klijenta

Da biste ponovo generisali API klijenta iz najnovije OpenAPI specifikacije:

1. Uverite se da imate FastComments server koji radi lokalno na `http://localhost:3001`
2. Pokrenite skriptu za ažuriranje:

```bash
./update.sh
```

Ovo će:
- Preuzeti najnoviju OpenAPI specifikaciju
- Generisati Swift klijentski kod (sa API dokumentacijom u client/docs)
- Izgraditi paket kako bi se provjerilo da sve radi