#### Odpri razvojne ključe v Canvas

Prijavite se v Canvas kot skrbnik. Pojdite na **Admin** (v levi stranski vrstici) > izberite svoj račun > **Developer Keys**.

#### Ustvarite LTI razvojni ključ

Kliknite **+ Developer Key** in izberite **LTI Key**.

V obrazcu za konfiguracijo:

1. V polje **Redirect URIs** (levo) prilepite **Launch URL** s strani za nastavitev FastComments.
2. Na desni strani nastavite **Method** na **Enter URL**.
3. Prilepite **Configuration URL**, ki ste ga kopirali iz FastComments, v polje **JSON URL**.
4. Canvas bo konfiguracijo LTI naložil samodejno.
5. Ključu dajte ime (npr. "FastComments").
6. Kliknite **Save**.

#### Omogočite razvojni ključ

Po shranjevanju se bo nov ključ prikazal v tabeli Developer Keys z **State** nastavljenim na **OFF**. Kliknite preklopni stikalo, da ga nastavite na **ON**. Canvas vas lahko pozove k potrditvi — kliknite **Allow**, da omogočite ključ.

#### Kopirajte Client ID

Tabela Developer Keys prikazuje številčni **Client ID** v stolpcu Details (npr. `17000000000042`). Kopirajte to številko — potrebna bo v naslednjem koraku.