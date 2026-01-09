FastComments ordnet SAML-Benutzerrollen internen Berechtigungen zu und ermöglicht rollenbasierte Zugriffskontrolle für Ihre Organisation.

### FastComments Rollen-System

FastComments verwendet ein rollenbasiertes Berechtigungssystem, bei dem Benutzer eine oder mehrere Rollen haben können, die ihre Zugriffsrechte und Funktionen bestimmen.

### Verfügbare FastComments-Rollen

#### Administrative Rollen

**fc-account-owner**
- **Berechtigungen**: Vollständiger administrativer Zugriff
- **Fähigkeiten**: Alle Funktionen, Abrechnungsverwaltung, Benutzerverwaltung
- **Anwendungsfall**: Haupt-Account-Administratoren und Eigentümer

**fc-admin-admin**  
- **Berechtigungen**: Administrativer Zugriff auf die meisten Funktionen
- **Fähigkeiten**: Benutzerverwaltung, Konfiguration, Moderation. **Kann andere Admins verwalten.**
- **Anwendungsfall**: Sekundäre Administratoren und IT-Mitarbeiter

**fc-billing-admin**
- **Berechtigungen**: Verwaltung von Abrechnung und Abonnements
- **Fähigkeiten**: Zahlungsmethoden, Rechnungen, Änderung von Abonnements
- **Anwendungsfall**: Mitglieder des Finanzteams und Abrechnungskontakte

#### Spezialisierte Rollen

**fc-analytics-admin**
- **Berechtigungen**: Zugriff auf Analytics und Berichte
- **Fähigkeiten**: Anzeigen von Seitenstatistiken, Daten zum Nutzerengagement
- **Anwendungsfall**: Marketingteams und Datenanalysten

**fc-api-admin**
- **Berechtigungen**: API-Zugriff und -Verwaltung
- **Fähigkeiten**: API-Zugangsdaten, Webhook-Konfiguration
- **Anwendungsfall**: Entwickler und technische Integratoren

**fc-moderator**
- **Berechtigungen**: Fähigkeiten zur Kommentar-Moderation
- **Fähigkeiten**: Kommentare genehmigen/ablehnen, Spam verwalten
- **Anwendungsfall**: Community-Moderatoren und Content-Manager

### Rollen-Mapping-Konfiguration

#### SAML-Attributquellen

FastComments akzeptiert Rolleninformationen aus verschiedenen SAML-Attributnamen, um die Kompatibilität mit unterschiedlichen Identitätsanbietern sicherzustellen:

**Standard-Attributnamen**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS-Attribute**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Unterstützte Rollenformate

**Array-Format** *(Bevorzugt)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Komma-separiertes Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Einzelnes Rollenformat**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Rollen-Konfiguration des Identitätsanbieters

#### Microsoft Azure AD

1. **App-Rollen-Konfiguration**:
   - Definieren Sie FastComments-Rollen in Ihrer Azure AD-Anwendung
   - Weisen Sie Benutzern die passenden App-Rollen zu
   - Konfigurieren Sie Claims, damit zugewiesene Rollen enthalten sind

2. **Attributzuordnung**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Gruppenzuordnung**:
   - Erstellen Sie Gruppen, die den FastComments-Rollennamen entsprechen
   - Weisen Sie Benutzer den passenden Gruppen zu
   - Konfigurieren Sie Attributanweisungen

2. **Attributanweisung**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Gruppenabbildung**:
   - Erstellen Sie Organisationseinheiten oder Gruppen
   - Benennen Sie Gruppen mit FastComments-Rollenpräfixen
   - Konfigurieren Sie die Attributzuordnung

2. **Benutzerdefinierte Attribute**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Standardverhalten von Benutzern

#### Benutzer ohne Rollen

Wenn ein SAML-Benutzer keine Rollen oder nicht erkannte Rollen hat:
- Der Benutzer wird als Standard-Kommentator erstellt
- Es wird kein administrativer Zugriff gewährt
- Kann eigene Kommentare posten und verwalten
- Kann nicht auf Funktionen des Administrations-Dashboards zugreifen

#### Rollenvererbung

- Benutzer können gleichzeitig mehrere Rollen haben
- Berechtigungen sind kumulativ (es gilt das höchste Berechtigungsniveau)
- Rollenänderungen im IdP werden beim nächsten Login übernommen

### Verwaltung von SAML-Benutzern

#### Benutzererstellung

Wenn sich ein Benutzer zum ersten Mal über SAML anmeldet:
1. **Benutzerkonto**: Wird automatisch mit der E-Mail als Identifikator erstellt
2. **Rollen-Zuweisung**: Rollen werden basierend auf den SAML-Attributen angewendet
3. **Profilinformationen**: Vor- und Nachname werden ausgefüllt, falls vorhanden
4. **Aktivierung der Berechtigungen**: Rollen werden sofort aktiviert

#### Rollenaktualisierungen

Bestehende SAML-Benutzer erhalten Rollenaktualisierungen:
1. **Login-Auslöser**: Rollenaktualisierungen erfolgen bei jedem SAML-Login
2. **Sofortige Wirkung**: Neue Berechtigungen gelten sofort
3. **Rollenentfernung**: Entfernte Rollen werden automatisch entzogen
4. **Prüfpfad**: Rollenänderungen werden in den Prüfprotokollen protokolliert

### Benutzerdefinierte Rollenabbildung

#### Anpassungen für Unternehmen

Für Unternehmenskunden mit speziellen Anforderungen:
- Benutzerdefinierte Rollennamen können FastComments-Berechtigungen zugeordnet werden
- Komplexe Rollenhierarchien können implementiert werden
- Abteilungsspezifische Zugriffskontrollen können konfiguriert werden

Kontaktieren Sie den FastComments-Support für Konfigurationen zur benutzerdefinierten Rollenabbildung.

#### Rollenvalidierung

FastComments validiert eingehende Rollen:
- Nicht erkannte Rollen werden ignoriert (nicht abgewiesen)
- Fehlerhaft formatierte Rollenattribute werden zur Fehlerbehebung protokolliert
- Benutzer behalten vorhandene Rollen, wenn die SAML-Aussage keine Rolleninformationen enthält

### Beste Vorgehensweisen

#### Rollenverwaltung

1. **Prinzip der geringsten Privilegien**: Weisen Sie nur die minimal notwendigen Berechtigungen zu
2. **Regelmäßige Prüfungen**: Überprüfen Sie Benutzerrollen und Zugriffe regelmäßig  
3. **Klare Benennung**: Verwenden Sie beschreibende Gruppennamen in Ihrem IdP
4. **Dokumentation**: Halten Sie Dokumentation der Rollenzuweisungen

#### Sicherheitsüberlegungen

1. **Rollenattribute**: Stellen Sie sicher, dass Rollenattribute in SAML-Antworten ordnungsgemäß gesichert sind
2. **Attributvalidierung**: Überprüfen Sie, dass nur autorisierte Systeme Rollen zuweisen können
3. **Zugriffsüberprüfungen**: Überprüfen Sie regelmäßig Zuordnungen administrativer Rollen
4. **Überwachung**: Überwachen Sie Rollenänderungen und administrative Aktionen

### Fehlerbehebung bei Rollenproblemen

#### Häufige Probleme

**Rollen nicht angewendet**:
- Überprüfen Sie, ob die SAML-Attributnamen mit den unterstützten Formaten übereinstimmen
- Stellen Sie sicher, dass der IdP Rolleninformationen sendet
- Bestätigen Sie, dass die Rollenwerte genau mit den FastComments-Rollennamen übereinstimmen

**Zugriff verweigert**:
- Überprüfen Sie, ob dem Benutzer im IdP die entsprechende Rolle zugewiesen ist
- Prüfen Sie Rechtschreibung und Groß-/Kleinschreibung der Rolle
- Bestätigen Sie, dass die Rolle in der SAML-Antwort korrekt formatiert ist

**Fehlende Berechtigungen**:
- Überprüfen Sie Rollendefinitionen und erforderliche Berechtigungen
- Prüfen Sie auf widersprüchliche Rollenzuweisungen
- Vergewissern Sie sich, dass sich der Benutzer nach Rollenänderungen angemeldet hat

---