### Genopbygning af klienten

For at genopbygge API-klienten ud fra den seneste OpenAPI-specifikation:

1. Sørg for, at FastComments-serveren kører lokalt på `http://localhost:3001`
2. Kør opdateringsscriptet:

```bash
./update.sh
```

Dette vil:
- Downloade den seneste OpenAPI-spec
- Generere Swift-klientkoden (med API-dokumentation i client/docs)
- Bygge pakken for at bekræfte, at alt fungerer