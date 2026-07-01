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

Voeg vervolgens de benodigde afhankelijkheden toe:

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

Voeg de Repsy-repository toe aan je **build.gradle**‑bestand:

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

Deze bibliotheek bevat drie modules. De gegenereerde API‑client, de kern‑Java‑bibliotheek die handgeschreven hulpprogramma’s bevat om het werken met de API gemakkelijker te maken, en de `pubsub`‑module die een bibliotheek is om je te abonneren op wijzigingsfeeds.

- [API‑clientbibliotheekdocumentatie](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentatie van de kernbibliotheek, inclusief SSO‑voorbeelden](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub‑bibliotheekdocumentatie](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Publieke versus beveiligde API's

Voor de API‑client zijn er drie klassen: `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die je API‑sleutel vereisen, en de `PublicApi` bevat methoden die rechtstreeks vanuit een browser, mobiel apparaat, enzovoort kunnen worden aangeroepen zonder authenticatie.

De `ModerationApi` biedt een uitgebreide reeks live en snelle moderatie‑API’s. Elke `ModerationApi`‑methode accepteert een `sso`‑parameter en kan authenticeren via SSO of een FastComments.com‑sessie‑cookie.