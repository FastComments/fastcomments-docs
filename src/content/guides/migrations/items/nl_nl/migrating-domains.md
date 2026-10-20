FastComments biedt een geautomatiseerde manier om uw opmerkingen over domeinen te migreren.

De domeinmigratie vereist simpelweg een `from`- en een `to`-domein.

Dit **verplaatst** opmerkingen, het kopieert ze niet. Als u opmerkingen wilt kopiëren, neem dan contact met ons op.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Domein migratietool met de van- en naar-domeinvvelden en het aantal verplaatste opmerkingen'; title='Migreren van domeinen' app-screenshot-end]

Dit is ook bijvoorbeeld handig als een deel van uw migratie naar FastComments het migreren van een andere provider omvat, zodat uw importgegevens van opmerkingen gegevens kunnen bevatten die gemigreerd moeten worden. In dat geval kunt u de import uitvoeren en vervolgens de domeinmigratie.

### Monitoring Progress

De Domeinmigratietool gebruikt hetzelfde FastComments jobverwerkingssysteem als de andere gegevensbeheertools.

Er kan een vertraging zijn voordat uw migratie start. Dit is normaal, omdat het systeem periodiek controleert op nieuwe taken om te verwerken.

Terwijl de taak loopt, wordt het aantal gevonden opmerkingen om te migreren en het aantal tot nu toe gemigreerde opmerkingen weergegeven.

### Migrating Domains via API

Domeinen kunnen ook worden gemigreerd met [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), waarbij `type` is ingesteld op `domain`. Voortgang is beschikbaar via [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), en de migratie kan gedurende 90 dagen ongedaan worden gemaakt.

---