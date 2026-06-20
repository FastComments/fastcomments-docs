### Maven

Voeg de Repsy-repository toe aan de POM van je project:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Voeg daarna de benodigde dependencies toe:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Voeg de Repsy-repository toe aan je build.gradle-bestand:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Deze bibliotheek bevat drie modules. De gegenereerde API-client, de core Java-bibliotheek die handgeschreven hulpprogramma's bevat om het werken met de API te vergemakkelijken, en de `pubsub`-module die een bibliotheek is voor het abonneren op wijzigingsfeeds.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Voor de API-client zijn er drie klassen, `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die je API-sleutel vereisen, en `PublicApi` bevat methoden die direct vanuit een browser/mobiel apparaat/etc kunnen worden aangeroepen zonder authenticatie.

De `ModerationApi` verzorgt het moderator-dashboard. Het bevat methoden voor commentaarmoderatie (lijst, aantal, zoeken, logboeken en export), moderatie-acties (verwijderen/terugzetten, markeren, review/spam/goedkeuringsstatus instellen, stemmen en draad heropenen/sluiten), bans (verbieden van commentaar, een ban ongedaan maken, pre-ban-samenvattingen, ban-status en voorkeuren, en tellingen van gebande gebruikers), en badges & vertrouwen (een badge toekennen/verwijderen, handmatige badges, trust factor ophalen/instellen en interne gebruikersprofiel). Elke `ModerationApi`-methode accepteert een `sso`-parameter zodat de oproep namens een via SSO geauthenticeerde moderator kan worden uitgevoerd.