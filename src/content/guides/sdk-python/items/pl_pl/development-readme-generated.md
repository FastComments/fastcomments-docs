### Uruchamianie testów

```bash
# Ustaw zmienne środowiskowe
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Uruchom testy
pytest tests/
```

### Regeneracja klienta

Aby wygenerować ponownie klienta API na podstawie najnowszej specyfikacji OpenAPI:

```bash
./update.sh
```

Spowoduje to:
1. Pobranie najnowszej specyfikacji OpenAPI z działającego serwera FastComments (lub użycie lokalnego pliku openapi.yaml)
2. Wygenerowanie kodu klienta Pythona
3. Spłaszczenie struktury katalogów
4. Usunięcie zbędnych plików konfiguracyjnych