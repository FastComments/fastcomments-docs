#### Otwórz klucze deweloperskie w Canvas

Zaloguj się do Canvas jako administrator. Przejdź do **Admin** (w lewym pasku bocznym) > wybierz swoje konto > **Developer Keys**.

#### Utwórz klucz deweloperski LTI

Kliknij **+ Developer Key** i wybierz **LTI Key**.

W formularzu konfiguracji:

1. W polu **Redirect URIs** (po lewej stronie) wklej **Launch URL** ze strony konfiguracji FastComments.
2. Po prawej stronie ustaw **Method** na **Enter URL**.
3. Wklej **Configuration URL** skopiowany z FastComments do pola **JSON URL**.
4. Canvas automatycznie załaduje konfigurację LTI.
5. Nadaj kluczowi nazwę (np. "FastComments").
6. Kliknij **Save**.

#### Włącz klucz deweloperski

Po zapisaniu nowy klucz pojawi się w tabeli Developer Keys ze stanem **State** ustawionym na **OFF**. Kliknij przełącznik, aby ustawić go na **ON**. Canvas może poprosić o potwierdzenie — kliknij **Allow**, aby włączyć klucz.

#### Skopiuj Client ID

Tabela Developer Keys pokazuje numeryczny **Client ID** w kolumnie Details (np. `17000000000042`). Skopiuj tę liczbę — będzie potrzebna w następnym kroku.