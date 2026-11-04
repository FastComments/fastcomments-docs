FastComments biedt een geautomatiseerde manier om je reacties tussen domeinen te migreren.

De domeinmigratie vereist slechts een `from`- en een `to`-domein.

Dit **verplaatst** reacties, het kopieert ze niet. Als je reacties wilt kopiëren, neem dan contact met ons op.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Domeinmigratietool met de from- en to-domeinvelden en het aantal gemigreerde reacties'; title='Domeinen migreren' app-screenshot-end]

Dit is ook nuttig als bijvoorbeeld een deel van je migratie naar FastComments het migreren van een andere provider omvat, zodat je importgegevens van reacties gegevens kunnen bevatten die gemigreerd moeten worden. In dat geval kun je eerst de import uitvoeren en daarna de domeinmigratie.

### Voortgang volgen

De Domain Migration-tool gebruikt hetzelfde FastComments-taakverwerkingssysteem als de andere gegevensbeheertools.

Er kan een vertraging zijn voordat je migratie start. Dit is normaal, omdat het systeem periodiek controleert op nieuwe taken om te verwerken.

Terwijl de taak draait, wordt het aantal gevonden te migreren reacties weergegeven, evenals het aantal tot nu toe gemigreerde reacties.

### Domeinen migreren via de API

Domeinen kunnen ook gemigreerd worden via [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), met `type` ingesteld op `domain`. De voortgang is beschikbaar via [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), en de migratie kan 90 dagen ongedaan worden gemaakt.