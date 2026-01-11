### Ponovno generiranje odjemalca

Za ponovno generiranje API odjemalca iz najnovejše specifikacije OpenAPI:

1. Prepričajte se, da imate lokalno zagnan FastComments strežnik na `http://localhost:3001`
2. Zaženite posodobitveni skript:

```bash
./update.sh
```

To bo:
- Prenese najnovejšo OpenAPI specifikacijo
- Ustvari Swift odjemalsko kodo (z API dokumentacijo v client/docs)
- Sestavi paket, da preveri, ali vse deluje