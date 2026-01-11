### Regeneracja klienta

Aby zregenerować klienta API z najnowszej specyfikacji OpenAPI:

1. Upewnij się, że serwer FastComments jest uruchomiony lokalnie pod adresem `http://localhost:3001`
2. Uruchom skrypt aktualizujący:

```bash
./update.sh
```

To spowoduje:
- Pobranie najnowszej specyfikacji OpenAPI
- Wygenerowanie kodu klienta Swift (z dokumentacją API w client/docs)
- Zbudowanie pakietu, aby zweryfikować, że wszystko działa